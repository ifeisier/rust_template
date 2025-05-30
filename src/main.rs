#![warn(missing_docs)]

//! 这是一个简单的 rust 模板项目.

mod application;
mod domain;
mod infrastructure;
mod interfaces;
mod shared;

use crate::interfaces::http::shared::HttpShared;
use crate::interfaces::http::start_http;
use shared::utils::flexi_logger::init_flexi_logger;
use std::io::Result;
use std::{process::exit, time::Duration};
use tokio::runtime::{Builder, Runtime};
use tokio::signal;

/// 主函数
fn main() {
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
    let http_shared = HttpShared::load().await;
    let http_shared = match http_shared {
        Ok(v) => v,
        Err(e) => {
            log::error!("load app_shared error: {}", e);
            exit(1);
        }
    };

    if let Err(e) = start_http(http_shared).await {
        log::error!("start http service error: {}", e);
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
