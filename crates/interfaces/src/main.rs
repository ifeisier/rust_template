#![warn(missing_docs)]

//! 暴露给外部的接口, 如 Web 服务.
//!
//! 在这里可以:
//!   - 读取配置文件
//!   - 配置依赖关系
//!   - 启动服务等

mod app_context;
mod http;

use crate::app_context::AppContext;
use dotenvy::from_filename;
use http::start_http;
use internal_core::mqtt_event::dispatch_mqtt_events;
use internal_shared::flexi_logger::init_flexi_logger;
use std::env;
use std::io::Result;
use std::process::exit;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::signal;

fn main() {
    // 决定环境 (默认 development)
    let env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());
    let env_file = format!(".env.{env}");
    from_filename(&env_file).ok();

    let logger = init_flexi_logger().unwrap();

    let runtime = new_multi_thread().unwrap();
    runtime.block_on(async_main());
    runtime.block_on(async move {
        signal::ctrl_c().await.unwrap();
        logger.flush();
        logger.shutdown();
    });
}

/// 异步执行入口
async fn async_main() {
    let app_context = AppContext::build().await;
    let mut app_context = match app_context {
        Ok(v) => v,
        Err(e) => {
            log::error!("build app context error: {e}");
            exit(1);
        }
    };

    if let Some(context) = app_context.mqtt_event_dispatch_context {
        dispatch_mqtt_events(context);
        app_context.mqtt_event_dispatch_context = None;
    }

    if let Err(e) = start_http(app_context).await {
        log::error!("start http service error: {e}");
        exit(1);
    }
}

/// 新建多线程运行时
#[allow(dead_code)]
fn new_multi_thread() -> Result<Runtime> {
    builder(Builder::new_multi_thread().worker_threads(5))
}

/// 使用当前线程新建运行时
#[allow(dead_code)]
fn new_current_thread() -> Result<Runtime> {
    builder(&mut Builder::new_current_thread())
}

/// 配置 Builder
fn builder(builder: &mut Builder) -> Result<Runtime> {
    builder
        .enable_all()
        .max_io_events_per_tick(2048)
        .max_blocking_threads(512)
        .thread_keep_alive(Duration::from_secs(60))
        .build()
}
