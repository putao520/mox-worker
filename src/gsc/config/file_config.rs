use std::fs::File;
use std::io::{Read};
use anyhow::{Result};
use serde::{Deserialize, Serialize};
use crate::gsc::mdl::redis::RedisConfig;
use crate::mox::account::MoxEndpoint;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileConfig {
    pub redis: RedisConfig,
    pub task: TaskConfig,   // 任务配置
    pub test: Option<MoxEndpoint>,  // 测试配置
}

// 任务的配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskConfig {
    pub max: u32,               // 最大并发任务数量
    pub interval: u64,          // 任务启动间隔(毫秒),账号登录的间隔
    pub retry_interval: u32,    // 任务启动间隔(秒),无可用账号时的重试间隔
    pub disable_assign: bool,   // 禁用领区分配
    pub disable_proxy: bool,    // 禁用代理
}

pub fn load_file_config() -> Result<FileConfig> {
    // 读取 cfg.yaml
    let mut file = File::open("cfg.yaml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config: FileConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}