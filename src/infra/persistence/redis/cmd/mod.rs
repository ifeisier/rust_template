//! 操作 redis 的命令
//!
//! 使用例子:
//!
//! ```norun
//! use crate::connection_pool;
//! use redis::Commands;
//!
//! fn example() {
//!     let mut conn = connection_pool::get_index0_conn().unwrap();
//!     let _: () = conn.set("my_key", 42).unwrap();
//! }
//! ```
