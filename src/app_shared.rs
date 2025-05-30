//! app 内共享数据.
//!
//! 用来组装 domain、infrastructure、interfaces 模块数据.

use crate::infrastructure::persistence::mysql::{self as _mysql, MySQLOptions};
use crate::infrastructure::persistence::redis::{self as _redis, RedisOptions};
use anyhow::Result;
use r2d2::Pool;
use redis::Client;

/// 需要在整个 app 中共享的数据.
#[allow(dead_code)]
pub struct AppShared {
    /// mysql 连接池
    pub mysql_pool: mysql::Pool,
    /// redis 连接池
    pub redis_pool: Pool<Client>,
}

impl AppShared {
    /// 加载
    pub async fn load() -> Result<Self> {
        Ok(Self {
            mysql_pool: Self::mysql()?,
            redis_pool: Self::redis()?,
        })
    }

    /// 创建 redis 连接池
    fn redis() -> Result<Pool<Client>> {
        let options = RedisOptions::from_file("./config/redis.yaml")?;
        log::info!("{:?}", options);
        _redis::create_connection_pool(options)
    }

    /// 创建 mysql 连接池
    fn mysql() -> Result<mysql::Pool> {
        let options = MySQLOptions::from_file("./config/mysql.yaml")?;
        log::info!("{:?}", options);
        _mysql::create_connection_pool(options)
    }
}
