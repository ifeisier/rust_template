//! Web 控制器, 接收用户请求调用对应服务, 并返回响应.

use axum::Router;
use axum::response::Json;
use axum::routing::get;
use serde_json::{Value, json};
use shared::HttpShared;
use std::sync::Arc;

pub mod shared;

/// 返回系统信息
pub async fn system_info() -> Json<Value> {
    Json(json!({"version": "1.0.0"}))
}

/// 启动 HTTP 服务.
pub async fn start_http(http_shared: HttpShared) -> anyhow::Result<()> {
    let http_shared = Arc::new(http_shared);
    let app = Router::new()
        .route("/system_info", get(system_info))
        .with_state(http_shared);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
