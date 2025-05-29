//! Web 控制器, 接收用户请求调用对应服务, 并返回响应.

use axum::response::Json;
use serde_json::{Value, json};

/// 返回系统信息
pub async fn system_info() -> Json<Value> {
    Json(json!({"version": "1.0.0"}))
}
