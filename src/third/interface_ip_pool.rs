use serde::{Deserialize, Serialize};
use anyhow::Result;

pub trait IpPoolServices {
    fn get_auth(&self) -> (String, String);
    fn is_auth(&self) -> bool;
    async fn get_ip(&self) -> Result<String>;
    async fn valid_ip(&self, ip: String)-> Result<bool>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IpInfo {
    pub ip: String,
    pub port: u32,
}