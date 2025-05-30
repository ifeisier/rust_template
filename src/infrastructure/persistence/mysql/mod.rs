//! 获取 mysql 连接、创建新的事务和 SQL 操作.

mod domain_sql;

use anyhow::{Result, anyhow};
use mysql::{Opts, OptsBuilder, Pool, PoolConstraints, PoolOpts};
use serde::Deserialize;
use std::collections::HashMap;
use xx_toolkit::yaml::from_yaml_file;

/// mysql 配置
#[derive(Debug, Deserialize)]
pub struct MySQLOptions {
    /// 主机
    pub host: String,
    /// 端口
    pub port: usize,
    /// 用户名
    pub user: String,
    /// 密码
    pub password: String,
    /// 数据库名
    pub db_name: String,
    /// TCP 连接的超时时间,单位是毫秒
    pub tcp_connect_timeout_ms: usize,
    /// 数据传输的超时时间,单位是毫秒
    #[allow(dead_code)]
    pub tcp_user_timeout_ms: usize,
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

/// 创建新的事务参数
#[allow(dead_code)]
pub fn new_transaction_options() -> mysql::TxOpts {
    mysql::TxOpts::default()
        .set_with_consistent_snapshot(true)
        .set_isolation_level(Some(mysql::IsolationLevel::RepeatableRead))
        .set_access_mode(Some(mysql::AccessMode::ReadWrite))
}

/// 创建 mysql 连接池
pub fn create_connection_pool(opt: MySQLOptions) -> Result<Pool> {
    let mut from_hash_map = HashMap::new();

    from_hash_map.insert("host".to_owned(), opt.host);
    from_hash_map.insert("port".to_owned(), opt.port.to_string());
    from_hash_map.insert("user".to_owned(), opt.user);
    from_hash_map.insert("password".to_owned(), opt.password);
    from_hash_map.insert("db_name".to_owned(), opt.db_name);

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    from_hash_map.insert("tcp_keepalive_probe_count".to_owned(), "6".to_owned());
    from_hash_map.insert("tcp_keepalive_time_ms".to_owned(), "5000".to_owned());

    // fast: 快速压缩, best: 最佳压缩(压缩比最高).
    from_hash_map.insert("compress".to_owned(), "best".to_owned());
    from_hash_map.insert("secure_auth".to_owned(), "true".to_owned());

    // TCP 连接的超时时间,单位是毫秒
    from_hash_map.insert(
        "tcp_connect_timeout_ms".to_owned(),
        opt.tcp_connect_timeout_ms.to_string(),
    );

    // 数据传输的超时时间,单位是毫秒
    #[cfg(target_os = "linux")]
    from_hash_map.insert(
        "tcp_user_timeout_ms".to_owned(),
        opt.tcp_user_timeout_ms.to_string(),
    );

    // 预处理语句缓存大小
    from_hash_map.insert(
        "stmt_cache_size".to_owned(),
        opt.stmt_cache_size.to_string(),
    );

    let pc = PoolConstraints::new(opt.pool_min, opt.pool_max).ok_or(anyhow!("创建连接池失败"))?;
    let opts_builder = OptsBuilder::new()
        .pool_opts(PoolOpts::default().with_constraints(pc))
        .from_hash_map(&from_hash_map)?;
    Ok(Pool::new(Opts::from(opts_builder))?)
}
