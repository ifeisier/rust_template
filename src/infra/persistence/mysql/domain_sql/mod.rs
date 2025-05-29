//! 数据库的表和结构体的映射以及 sql 都放在这么模块中.
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
