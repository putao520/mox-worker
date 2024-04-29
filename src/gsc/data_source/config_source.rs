use std::sync::RwLock;

use anyhow::Result;
use futures::StreamExt;
use once_cell::sync::Lazy;
use redis::Commands;

use crate::gsc::config::system_config::SystemConfig;
use crate::gsc::event::pub_sub::EventSystem;
use crate::gsc::mdl::redis::get_sync_redis_connect;
use crate::gsc::time_until::delay_max_secs;

// 线程安全的 SystemConfig
static SYSTEM_CONFIG_INSTANCE: Lazy<RwLock<SystemConfig>> = Lazy::new(||{
    let mut con = get_sync_redis_connect().unwrap();
    let res = con.get::<&str, SystemConfig>("config");
    let cfg = res.unwrap_or_else(|_| SystemConfig::new());

    RwLock::new(cfg)
});

// 提供配置数据源管理
pub fn get_share_config() -> Result<SystemConfig> {
    Ok(SYSTEM_CONFIG_INSTANCE.read().unwrap().clone())
}

// 写入配置
pub fn set_share_config(cfg: &SystemConfig) -> Result<()> {
    if let Ok(mut write_guard) = SYSTEM_CONFIG_INSTANCE.write() {
        let mut con = get_sync_redis_connect()?;
        con.set("config", cfg.clone())?;
        *write_guard = cfg.clone();
    }
    Ok(())
}

pub fn del_share_config() -> Result<()> {
    let mut con = get_sync_redis_connect()?;
    con.del::<_,usize>("config")?;
    Ok(())
}

fn sync_share_config(msg: &String) -> Result<()> {
    println!("share_config: changed");
    let cfg: SystemConfig = serde_json::from_str(msg)?;
    set_share_config(&cfg)
}

pub async fn clear_share_config() -> Result<()> {
    let mut con = get_sync_redis_connect()?;
    con.del::<_,usize>("config")?;
    Ok(())
}

// 监视任务->外部配置变化
pub async fn auto_change_share_config()->EventSystem{
    let mut es = EventSystem::new();
    es.subscribe("#config_changed", sync_share_config).await.expect("subscribe error");
    es
}

#[cfg(test)]
mod tests {
    use redis::AsyncCommands;

    use crate::gsc::config::system_config::{AccountConfig, CaptchaConfig, MoxClientConfig, ProxyConfig, S3Config};
    use crate::gsc::mdl::redis::get_redis_pub;
    use crate::gsc::time_until::delay_max_secs;

    use super::*;

    #[test]
    fn test_share_config() {
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
                url: "".to_string(),
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

        // 测试读取配置
        let mut shara_cfg = get_share_config().unwrap();
        assert_eq!(shara_cfg.account.max, 5);

        // 测试写入配置
        shara_cfg.account.max = 10;
        set_share_config(&shara_cfg).unwrap();
        let shara_cfg = get_share_config().unwrap();
        assert_eq!(shara_cfg.account.max, 10);

        // 还原配置
        let mut con = get_sync_redis_connect().unwrap();
        con.del::<_,usize>("config").unwrap();
    }

    #[tokio::test]
    async fn test_config_notify() {
        clear_share_config().await.unwrap();
        let es = auto_change_share_config().await;

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
                url: "".to_string(),
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

        let mut cfg = get_share_config().unwrap();
        cfg.account.max = 100;
        set_share_config(&cfg).unwrap();

        let mut ps = get_redis_pub().await.unwrap();
        let _: usize = ps.publish("#config_changed", &cfg).await.unwrap();
        delay_max_secs(10).await;
        let cfg = get_share_config().unwrap();
        assert_eq!(cfg.account.max, 100);
    }
}