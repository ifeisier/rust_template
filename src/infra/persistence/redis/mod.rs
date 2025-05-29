//! 获取 redis 连接、创建 redis 脚本和 redis 操作.

mod cmd;
mod script;

use anyhow::Result;
use r2d2::Pool;
use redis::{Client, ConnectionAddr, ConnectionInfo, ProtocolVersion, RedisConnectionInfo};
use serde::Deserialize;
use xx_toolkit::yaml::from_yaml_file;

/// redis 配置
#[derive(Debug, Deserialize)]
pub struct RedisOptions {
    /// 主机
    pub host: String,
    /// 端口
    pub port: u16,
    /// 密码
    pub password: Option<String>,
    /// 数据库
    pub db: i64,
    /// 连接池最小连接数
    pub pool_min: u32,
    /// 连接池最大连接数
    pub pool_max: u32,
}
impl RedisOptions {
    /// 从文件加载配置.
    pub fn from_file(path: &str) -> Result<Self> {
        from_yaml_file(path)
    }
}

/// 创建 redis 连接池
pub fn create_connection_pool(opt: RedisOptions) -> Result<Pool<Client>> {
    let info = ConnectionInfo {
        addr: ConnectionAddr::Tcp(opt.host, opt.port),
        redis: RedisConnectionInfo {
            db: opt.db,
            username: None,
            password: opt.password,
            protocol: ProtocolVersion::default(),
        },
    };

    let mut builder = r2d2::Builder::new();
    builder = builder.max_size(opt.pool_max);
    builder = builder.min_idle(Some(opt.pool_min));
    Ok(builder.build(Client::open(info)?)?)
}
