//! redis 脚本
//!
//! 使用例子:
//!
//! ```norun
//! use redis::Client;
//! use r2d2::PooledConnection;
//!
//! fn example(conn: &mut PooledConnection<Client>) -> Result<Option<String>, redis::RedisError> {
//!     let args = vec!["arg1", "arg2"];
//!
//!     let script = creat_script("");
//!     script.key("key1");
//!     script.key("key2");
//!
//!     for ele in args {
//!         script.arg(ele);
//!     }
//!
//!     script.invoke::<Option<String>>(conn)
//! }
//! ```

use redis::Script;
use std::path::Path;

/// 创建 redis 脚本
#[allow(dead_code)]
pub fn creat_script<P: AsRef<Path>>(path: P) -> Script {
    let message = std::fs::read_to_string(path).unwrap();
    redis::Script::new(message.as_str())
}
