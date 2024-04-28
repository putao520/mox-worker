use std::str::FromStr;
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use crate::gsc::config::system_config::ProxyConfig;
use crate::third::interface_ip_pool::{IpInfo, IpPoolServices};
use anyhow::{anyhow, Result};
use crossbeam_queue::ArrayQueue;
use log::{error, info};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use crate::gsc::error::{Error};
use crate::gsc::time_until::delay_min_max_secs;

static IP_POOL: Lazy<ArrayQueue<IpInfo>> = Lazy::new(||{
    ArrayQueue::new(10)
});

static GLOBAL_UPDATE_LOCK: Lazy<Mutex<bool>> = Lazy::new(||{
    Mutex::new(false)
});

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct RoxLabs {
    pub host: String,
    pub port: u32,
    pub proxy_str: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct RoxLabsResponse {
    msg: String,
    code: u32,
    success: bool,
    data: Vec<IpInfo>,
}

impl RoxLabs {
    pub fn new(cfg: &ProxyConfig) -> Self {
        // http://api.tq.roxlabs.cn/getProxyIp?num=10&return_type=json&lb=1&sb=&flow=1&regions=&protocol=http
        let mut host_str = format!("http://{}", cfg.host);
        if cfg.port != 80 {
            host_str.push_str(&format!(":{}", cfg.port));
        }
        RoxLabs {
            host: cfg.host.clone(),
            port: cfg.port,
            proxy_str: format!("{}/getProxyIp?num=10&return_type=json&lb=1&sb=0&flow=1&regions=&protocol=http", host_str),
        }
    }
}

impl IpPoolServices for RoxLabs {

    fn get_auth(&self) -> (String,String) {
        ("".to_string(), "".to_string())
    }

    fn is_auth(&self) -> bool {
        true
    }

    async fn get_ip(&self) -> Result<String> {
        let mut err_no = 5;
        loop {
            match IP_POOL.pop() {
                Some(pool) => {
                    return Ok(format!("http://{}:{}", pool.ip, pool.port.to_string()));
                },
                None => {
                    let _ = GLOBAL_UPDATE_LOCK.lock().await;
                    if IP_POOL.is_empty() {
                        let res = reqwest::get(&self.proxy_str).await?;
                        let str = res.text().await?;
                        #[cfg(debug_assertions)]
                        info!("get_ip_res: {:?}", str);
                        let v = serde_json::Value::from_str(str.as_str()).unwrap();
                        let success = v["success"].as_bool().unwrap();
                        if success {
                            let data = v["data"].as_array().unwrap();
                            for ip in data {
                                let ip_info = IpInfo {
                                    ip: ip["ip"].as_str().unwrap().to_string(),
                                    port: ip["port"].as_u64().unwrap() as u32,
                                };
                                let _ = IP_POOL.push(ip_info);
                            }
                        } else {
                            let msg = v["msg"].as_str().unwrap_or_else(|| "no msg");
                            error!("get_ip error: {:?}", msg);
                            return Err(anyhow!(Error{no:0x42001, msg: format!("IP服务故障:{}", msg).to_string() }))
                        }
                        delay_min_max_secs(0, 2).await;
                    }
                }
            }
            err_no -= 1;
            if err_no == 0 {
                return Err(anyhow!(Error{no:0x42001, msg: "IP服务未知异常!".to_string() }))
            }
        }
    }

    async fn valid_ip(&self, _ip: String) -> Result<bool> {
        Ok(true)
    }
}