use std::sync::OnceLock;
use redis::aio::{ConnectionManager, PubSub};
use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Result};
use dashmap::DashMap;
use log::info;
use once_cell::sync::Lazy;
use crate::gsc::config::file_config::{FileConfig, load_file_config};

/// Redis连接信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    /// 主机名，例如："localhost"
    pub host: String,
    /// 端口号，例如：6379
    pub port: u16,
    /// 用户名，例如："your_username"
    pub username: Option<String>,
    /// 密码，例如："your_password"
    pub password: Option<String>,
}

static REDIS_CONFIG: OnceLock<RedisConfig> = OnceLock::new();

static REDIS_CLIENTS: Lazy<DashMap<u8,&'static redis::Client>> = Lazy::new(|| {
    DashMap::new()
});

pub fn get_redis_client(db_id: u8) -> &'static redis::Client {
    REDIS_CLIENTS.entry(db_id).or_insert_with(|| {
        let cfg = REDIS_CONFIG.get_or_init(|| {
            let local_config = load_file_config().unwrap_or_else(|e| {
                panic!("读取本地配置文件错误: {}", e)
            });
            local_config.redis
        });
        let username = cfg.username.clone().unwrap_or_else(|| "".to_string());
        let password = cfg.password.clone().unwrap_or_else(|| "".to_string());
        let redis_uri = format!("redis://{}:{}@{}:{}/{}", username, password, cfg.host, cfg.port, db_id);
        info!("redis_uri: {}", redis_uri);
        Box::leak(Box::new(redis::Client::open(redis_uri).unwrap_or_else(
            |e| panic!("创建Redis客户端错误: {}", e)
        )))
    }).value()
}

pub async fn get_redis_connect() -> Result<ConnectionManager> {
    get_redis_client(0).get_connection_manager().await.map_err(|e| anyhow!(e))
}

pub async fn get_redis_connect_ex(db_id: u8) -> Result<ConnectionManager> {
    get_redis_client(db_id).get_connection_manager().await.map_err(|e| anyhow!(e))
}

pub fn get_sync_redis_connect() -> Result<redis::Connection> {
    get_redis_client(0).get_connection().map_err(|e| anyhow!(e))
}

pub async fn get_redis_sub() -> Result<PubSub> {
    get_redis_client(10).get_async_pubsub().await.map_err(|e| anyhow!(e))
}

pub async fn get_redis_pub() -> Result<ConnectionManager> {
    get_redis_client(10).get_connection_manager().await.map_err(|e| anyhow!(e))
}

pub async fn get_redis_pub_sub_ex(db_id: u8) -> Result<PubSub> {
    get_redis_client(db_id).get_async_pubsub().await.map_err(|e| anyhow!(e))
}
