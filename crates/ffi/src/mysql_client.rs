//! 获取 mysql 连接、创建新的事务和 SQL 操作.
//!
//! 使用例子:
//!
//! ```no-run
//! use crate::connection_pool::get_master_connection;
//!
//! use anyhow::{bail, Result};
//! use mysql::prelude::*;
//! use mysql::*;
//!
//! #[derive(Debug, Clone, Default)]
//! pub struct Example {
//!     name: String,
//! }
//!
//! impl FromRow for Example {
//!     fn from_row_opt(row: Row) -> Result<Self, FromRowError>
//!     where
//!         Self: Sized,
//!     {
//!         let mut example = Example::default();
//!
//!         for (i, column) in row.columns_ref().iter().enumerate() {
//!             match column.name_str().as_ref() {
//!                 "name" => crate::extract_field!(row, i, example, name, String),
//!                 v => {
//!                     log::error!("未知字段: {}", v);
//!                     return Err(FromRowError(row));
//!                 }
//!             }
//!         }
//!
//!         Ok(example)
//!     }
//! }
//!
//! pub struct ExampleSql;
//! impl ExampleSql {
//!     pub async fn test() -> Result<Example> {
//!         let mut conn = get_master_connection()?;
//!
//!         let sql = "SELECT * FROM xx WHERE x1 = :x1;";
//!         let result = conn.exec_first_opt::<Example, _, _>(sql, params! {"x1" => "v1"})?;
//!
//!         match result {
//!             Some(v) => Ok(v?),
//!             None => bail!("没有找到设备"),
//!         }
//!     }
//! }
//!
//! // 也可以使用下面这种方式
//! pub async fn test<Q>(conn: &mut Q)
//! where
//!     Q: mysql::prelude::Queryable,
//! ```

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

/// 将 mysql 中的字段值转换为 rust 中的字段值
#[macro_export]
macro_rules! extract_field {
    ($row:expr, $i:expr, $struct:ident, $field:ident, $type:ty) => {
        if let Some(result) = $row.get_opt::<$type, _>($i) {
            match result {
                Ok(v) => $struct.$field = v,
                Err(e) => {
                    log::error!("解析 {} 字段错误: {:?}", stringify!($field), e);
                    return Err(FromRowError($row));
                }
            }
        }
    };
}
