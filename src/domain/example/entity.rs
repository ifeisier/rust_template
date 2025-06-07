//! example 相关实体.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Example 实体.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub value: Value,
}
