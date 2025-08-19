//! 启动 HTTP 服务端, 以及提供暴露给外部的接口.

use crate::app_context::AppContext;
use axum::Router;
use axum::response::Json;
use axum::routing::get;
use serde_json::{Value, json};
use std::sync::Arc;

/// 返回系统信息
pub async fn system_info() -> Json<Value> {
    Json(json!({"version": "1.0.0"}))
}

/// 启动 HTTP 服务.
pub async fn start_http(app_context: AppContext) -> anyhow::Result<()> {
    let http_shared = Arc::new(app_context);
    let app = Router::new()
        .route("/system_info", get(system_info))
        .with_state(http_shared);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
