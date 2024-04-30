use std::num::NonZeroUsize;

use anyhow::Result;
use futures::StreamExt;
use redis::{AsyncCommands, AsyncIter};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

use crate::gsc::config::system_config::AccountConfig;
use crate::gsc::data_source::account::AccountType;
use crate::gsc::data_source::source_service::AsStringEnum;
use crate::gsc::mdl::redis::{get_redis_connect, get_redis_connect_ex, get_redis_sub};

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone, Debug)]
pub struct MoxAccount {
    pub email: String,
    pub password: String,
    pub log_message: String,
    pub mox_endpoint: MoxEndpoint,
    // 过期时间戳
    pub expire_time: i64,
}

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone, Debug)]
pub struct MoxEndpoint {
    // 国家编号
    pub country_id: u32,
    // 省/州编号
    pub state_id: u32,
    // 城市编号
    pub city_code: Option<String>,
    // 办公室编号
    pub office_id: u32,
}

impl MoxEndpoint {
    pub fn new_empty() -> Self {
        MoxEndpoint {
            country_id: 0,
            state_id: 0,
            city_code: None,
            office_id: 0,
        }
    }
}

impl MoxAccount {
    pub fn new(email: String, password: String, mox_endpoint: &MoxEndpoint) -> Self {
        MoxAccount {
            email,
            password,
            log_message: "".to_string(),
            mox_endpoint: mox_endpoint.clone(),
            expire_time: 0,
        }
    }
}

static ACCOUNT_DB_NO: u8 = 5;

// 获得一个可用的账号
pub async fn get_valid_account(cfg: &AccountConfig) -> Result<MoxAccount> {
    let mut cli = get_redis_connect().await?;
    let mut cli_ex = get_redis_connect_ex(ACCOUNT_DB_NO).await?;
    loop {
        let v: Vec<MoxAccount> = cli.lpop(AccountType::Valid.as_str(), NonZeroUsize::new(1)).await?;
        if v.is_empty() {
            return Err(anyhow::anyhow!("无可用账号"));
        }
        let r = v.get(0).unwrap().clone();
        let b: bool = cli_ex.exists(r.email.clone()).await?;
        if b {
            continue;
        }
        cli.hset(AccountType::Using.as_str(), r.email.clone(), r.clone()).await?;
        cli_ex.set_ex(r.email.clone(), r.clone(), cfg.cool_down as u64).await?;
        return Ok(r.clone());
    }
}

// 封禁账号(必须先从 "use_account" 中获得账号数据)
pub async fn ban_account(account: &MoxAccount) -> Result<()> {
    let mut cli = get_redis_connect().await?;
    cli.hdel(AccountType::Using.as_str(), account.email.clone()).await?;
    let mut cli_ex = get_redis_connect_ex(ACCOUNT_DB_NO).await?;
    cli_ex.del(account.email.clone()).await?;
    cli.lpush(AccountType::Ban.as_str(), account.email.clone()).await?;
    Ok(())
}

// 过载账号(必须先从 "use_account" 中获得账号数据)
pub async fn overload_account(account: &MoxAccount) -> Result<()> {
    let mut cli_ex = get_redis_connect_ex(ACCOUNT_DB_NO).await?;
    // 获得当前时间戳
    let now = chrono::Utc::now().timestamp();
    let expire_time = account.expire_time - now;
    let expire_time = if expire_time > 0 {
        expire_time
    } else {
        0
    };
    cli_ex.expire(account.email.clone(), expire_time).await?;
    Ok(())
}

// 清理全部账号有关数据
static CACHE_MAX:usize = 100;
pub async fn clear_account() -> Result<()> {
    let mut cli = get_redis_connect().await?;
    cli.del(AccountType::Valid.as_str()).await?;
    cli.del(AccountType::Using.as_str()).await?;
    cli.del(AccountType::Ban.as_str()).await?;
    // 遍历 db 5 所有 key
    let mut cli_ex = get_redis_connect_ex(ACCOUNT_DB_NO).await?;
    let mut cli_del = get_redis_connect_ex(ACCOUNT_DB_NO).await?;
    let mut iter: AsyncIter<String> = cli_ex.scan().await?;
    let mut ca_list: Vec<String> = Vec::new();
    loop{
        loop {
            if let Some(key) = iter.next().await {
                ca_list.push(key);
            } else {
                break
            }
            if ca_list.len() >= CACHE_MAX {
                break
            }
        }
        if ca_list.is_empty() {
            break
        }
        for key in &ca_list {
            cli_del.del(key).await?;
        }
        ca_list.clear();
    }
    Ok(())
}

// 进程启动->遍历using账号,如果不在db5中,则加入valid
pub async fn rebuild_account()->Result<()>{
    let mut cli = get_redis_connect().await?;
    let mut cli_op = get_redis_connect().await?;
    let mut cli_ex = get_redis_connect_ex(ACCOUNT_DB_NO).await?;
    // 遍历 AccountType::Using 的 hash
    let mut iter: AsyncIter<String> = cli.hscan(AccountType::Using.as_str()).await?;
    while let Some(key) = iter.next().await {
        let b: bool = cli_ex.exists(key.clone()).await?;
        if !b {
            // 不存在,则加入valid
            let account: MoxAccount = cli_op.hget(AccountType::Using.as_str(), key.clone()).await?;
            let _: usize = cli_op.lpush(AccountType::Valid.as_str(), account).await?;
        }
    }
    cli_op.del(AccountType::Using.as_str()).await?;
    Ok(())
}


// 监视任务->检查账号是否到期
pub fn auto_recover_account(){
    let channel = format!("__keyevent@{}__:expired", ACCOUNT_DB_NO);
    tokio::spawn(async move {
        let mut pubsub = get_redis_sub().await.expect("get_redis_pub_sub error");
        pubsub.subscribe(channel).await.expect(format!("subscribe __keyevent {} error", ACCOUNT_DB_NO).as_str());
        let mut stream = pubsub.on_message();
        loop {
            if let Some(msg) = stream.next().await {
                if let Ok(payload) = msg.get_payload::<String>() {
                    println!("Key expired: {}", payload);
                    if let Ok(mut cli) = get_redis_connect().await {
                        if let Ok(account) = cli.hget::<_, _, MoxAccount>(AccountType::Using.as_str(), payload.clone()).await {
                            let _: usize = cli.lpush(AccountType::Valid.as_str(), account).await.expect("lpush error");
                            let _: usize = cli.hdel(AccountType::Using.as_str(), payload.clone()).await.expect("hdel error");
                        }
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use redis::AsyncCommands;

    use crate::gsc::config::system_config::{AccountConfig, CaptchaConfig, MoxClientConfig, ProxyConfig, S3Config, SystemConfig};
    use crate::gsc::data_source::account::AccountType;
    use crate::gsc::data_source::config_source::{get_share_config, set_share_config};
    use crate::gsc::data_source::source_service::AsStringEnum;
    use crate::gsc::mdl::redis::get_redis_connect;
    use crate::gsc::time_until::delay_min_max_secs;
    use crate::mox::account::{auto_recover_account, get_valid_account, MoxAccount, MoxEndpoint};

    fn build_config() {
        // 初始系统配置并写入
        let cfg = SystemConfig{
            account: AccountConfig{
                max: 5,
                appointment_period: 3,
                cool_down: 3600,
            },
            mox_client: MoxClientConfig {
                api_key: "VcAmGCISOnRc6AA".to_string(),
                offices_console_keys: "M5hxYq16KRyKfGHSlKzf4d7I92SUwBA02s6fxZg4YGkgsT4sEm2kME5L1alrpB8LuVxjawsGvojISFpRzZGjcDA8ELk9a1xTJKUk".to_string(),
                date_period: 5,
                time_period: 5,
            },
            s3_config: S3Config{
                region: "us-east-1".to_string(),
                endpoint: "localhost:9000".to_string(),
                access_key: "2IiYe6HtY8RpF8R2j6bi".to_string(),
                secret_key: "hv94WgGAsqTO6YCujEXsN6ZGovKy7OW1edzz6xCV".to_string(),
            },
            captcha: CaptchaConfig {
                url: "http://150.138.84.183:9896/ocr/b64".to_string(),
                token: "".to_string(),
                log_message: "".to_string()
            },
            proxy: ProxyConfig{
                host: "api.proxy.ipidea.io".to_string(),
                name: "".to_string(),
                password: "".to_string(),
                port: 80,
                log_message: "".to_string(),
            },
            log_message: "".to_string(),
        };
        set_share_config(&cfg).unwrap();
    }

    fn build_account_info() -> MoxAccount {
        MoxAccount {
            email: "JohnstonEhmer46@gmail.com".to_string().to_lowercase(),
            password: "Qi2014hai@@@@".to_string(),
            log_message: "".to_string(),
            mox_endpoint: MoxEndpoint {
                country_id: 14,     // 国家
                state_id: 3380,     // 州
                city_code: None,
                office_id: 223,     // 领事馆
            },
            expire_time: 0,
        }
    }

    #[tokio::test]
    async fn test_get_valid_account() {
        build_config();

        let system_config = get_share_config().expect("get_share_config error");

        // clear_account().await.expect("clear_account error");

        auto_recover_account();

        let account = build_account_info();
        let mut cli = get_redis_connect().await.expect("get_redis_connect error");
        let _: usize = cli.lpush(AccountType::Valid.as_str(), account).await.expect("lpush error");

        loop{
            match get_valid_account(&system_config.account).await {
                Ok(account) => {
                    println!("account: {:?}", account);
                    delay_min_max_secs(5, 5).await;
                },
                Err(e) => {
                    println!("error: {:?}", e);
                    break;
                }
            }
        }

        delay_min_max_secs(3600, 3600).await;
    }
}