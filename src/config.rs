use std::{env, fs::File};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// 应用程序配置结构
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// 服务器配置
    pub server: ServerConfig,
}

/// 服务器配置结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    /// 服务器主机地址
    pub host: String,
    /// 服务器端口号
    pub port: u16,
}

impl AppConfig {
    /// 加载配置文件
    ///
    /// 按以下优先级查找配置文件：
    /// 1. 当前目录下的 app.yaml
    /// 2. /etc/config/app.yaml
    /// 3. 环境变量 CHAT_CONFIG 指定的路径
    ///
    /// 如果以上位置都找不到配置文件，将返回错误
    pub fn load() -> Result<Self> {
        let ret = match (
            File::open("app.yaml"),
            File::open("/etc/config/app.yaml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(file), _, _) => serde_yaml::from_reader(file),
            (_, Ok(file), _) => serde_yaml::from_reader(file),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            (Err(_), Err(_), Err(_)) => bail!("config file not found"),
        };
        Ok(ret?)
    }
}
