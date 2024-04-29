use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tokio::fs::read_to_string;
use crate::gsc::data_source::config_source::set_share_config;

// 系统配置
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct SystemConfig {
    pub account: AccountConfig, // 预约账号配置
    pub mox_client: MoxClientConfig, // 预约客户端配置
    pub s3_config: S3Config, // 储存配置
    pub captcha: CaptchaConfig, // 打码配置
    pub proxy: ProxyConfig, // ip池配置
    pub log_message: String, // 日志信息
}


// 每个预约任务的账号配置
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct AccountConfig {
    pub max: u32,                   // 每个账号最大预约数量   5
    pub appointment_period: u32,    // 同一账号下,每个客户预约间隔(秒)
    pub cool_down: u32,             // 账号使用冷却时间(秒)
}

// 通讯配置
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct MoxClientConfig {
    pub api_key: String,
    pub offices_console_keys: String,
    pub date_period: u32,           // 可用日期查询间隔
    pub time_period: u32,           // 可用时间查询间隔
}

// 储存配置
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct S3Config {
    pub region: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

// 打码配置
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct CaptchaConfig {
    pub url: String,
    pub token: String,
    pub log_message: String,
}

// ip池配置
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct ProxyConfig {
    pub name: String,
    pub password: String,
    pub host: String,
    pub port: u32,
    pub log_message: String,
}

impl SystemConfig {
    // 创建空的 SystemConfig
    pub fn new() -> SystemConfig {
        SystemConfig {
            account: AccountConfig {
                max: 0,
                appointment_period: 0,
                cool_down: 0,
            },
            mox_client: MoxClientConfig {
                api_key: "".to_string(),
                offices_console_keys: "".to_string(),
                date_period: 0,
                time_period: 0,
            },
            s3_config: S3Config {
                region: "".to_string(),
                endpoint: "".to_string(),
                access_key: "".to_string(),
                secret_key: "".to_string(),
            },
            captcha: CaptchaConfig {
                url: "".to_string(),
                token: "".to_string(),
                log_message: "".to_string(),
            },
            proxy: ProxyConfig {
                name: "".to_string(),
                password: "".to_string(),
                host: "".to_string(),
                port: 0,
                log_message: "".to_string(),
            },
            log_message: "".to_string(),
        }
    }
}

pub async fn get_system_config() -> Result<SystemConfig> {
    // 异步读取文件内容
    let content = read_to_string("file_mode/system_config.yaml").await?;

    // 将字符串内容解析为 SystemConfig 类型
    let config: SystemConfig = serde_yaml::from_str(&content)?;

    set_share_config(&config)?;

    Ok(config)
}