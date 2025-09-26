//! 获取 mysql 连接、创建新的事务和 SQL 操作.

pub(crate) mod domain_sql;

use anyhow::{Result, anyhow};
use mysql_async::{Compression, Opts, OptsBuilder, Pool, PoolConstraints, PoolOpts};
use serde::Deserialize;
use xx_toolkit::yaml::from_yaml_file;

/// mysql 配置
#[derive(Debug, Deserialize)]
pub struct MySQLOptions {
    /// 主机
    pub host: String,
    /// 端口
    pub port: u16,
    /// 用户名
    pub user: String,
    /// 密码
    pub password: String,
    /// 数据库名
    pub db_name: String,
    /// 预处理语句缓存大小
    pub stmt_cache_size: usize,
    /// 连接池最小连接数
    pub pool_min: usize,
    /// 连接池最大连接数
    pub pool_max: usize,
}
impl MySQLOptions {
    /// 从文件加载配置.
    pub fn from_file(path: &str) -> Result<Self> {
        from_yaml_file(path)
    }
}

/// 创建 mysql 连接池
pub fn create_connection_pool(opt: MySQLOptions) -> Result<Pool> {
    let pc = PoolConstraints::new(opt.pool_min, opt.pool_max)
        .ok_or_else(|| anyhow!("创建连接池失败"))?;

    let opts_builder = OptsBuilder::default()
        .pool_opts(PoolOpts::default().with_constraints(pc))
        .ip_or_hostname(opt.host)
        .tcp_port(opt.port)
        .user(Some(opt.user))
        .pass(Some(opt.password))
        .db_name(Some(opt.db_name))
        .tcp_keepalive(Some(5000u32))
        .compression(Compression::best())
        .secure_auth(true)
        .stmt_cache_size(opt.stmt_cache_size);

    Ok(Pool::new(Opts::from(opts_builder)))
}
