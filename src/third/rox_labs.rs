use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use crate::gsc::config::system_config::ProxyConfig;
use crate::third::interface_ip_pool::IpPoolServices;
use anyhow::Result;
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct RoxLabs {
    pub name: String,
    pub password: String,
    pub host: String,
    pub port: u32,
    pub proxy_str: String,
}

impl RoxLabs {
    pub fn new(proxy_cfg: &ProxyConfig) -> Self {
        let proxy_str = format!("http://{}:{}", proxy_cfg.host, proxy_cfg.port);
        RoxLabs {
            name: proxy_cfg.name.clone(),
            password: proxy_cfg.password.clone(),
            host: proxy_cfg.host.clone(),
            port: proxy_cfg.port,
            proxy_str,
        }
    }
}

impl IpPoolServices for RoxLabs {

    fn get_auth(&self) -> (String,String) {
        // format!("{}:{}", self.name, self.password)
        (self.name.clone(), self.password.clone())
    }

    fn is_auth(&self) -> bool {
        self.name.len() > 0 && self.password.len() > 0
    }

    async fn get_ip(&self) -> Result<String> {
        Ok(self.proxy_str.clone())
    }

    async fn valid_ip(&self, _ip: String) -> Result<bool> {
        Ok(true)
    }
}