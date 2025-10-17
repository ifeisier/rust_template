#![warn(missing_docs)]

//! 调用外部系统 (FFI, gRPC, 数据库访问, HTTP 客户端等)

pub mod impls;
pub mod mqtt_client;
mod mysql_client;
mod redis_client;

use crate::mqtt_client::{MQTTV5Client, MqttClientOptions};
use crate::mysql_client::MySQLOptions;
use crate::redis_client::RedisOptions;
use anyhow::Result;
use redis::Client;
use rumqttc::v5::{AsyncClient, Event};
use tokio::sync::mpsc;

/// 初始化 Mqtt 客户端.
///
/// # Arguments
///
/// * `path` - 配置文件路径, 包含 MQTT 客户端的连接信息.
///
/// # Errors
///
/// 如果读取配置文件失败, 或者 MQTT 客户端初始化失败, 会返回相应的错误.
pub async fn init_mqtt_client(path: &str) -> Result<(AsyncClient, mpsc::Receiver<Event>)> {
    let opt = MqttClientOptions::from_file(path)?;
    MQTTV5Client::connect(opt).await
}

/// 初始化 Redis 连接池.
///
/// # Arguments
///
/// * `path` - 配置文件路径, 包含 Redis 连接信息.
///
/// # Errors
///
/// 如果读取配置文件失败, 或者创建连接池失败, 会返回相应的错误.
pub fn init_redis(path: &str) -> Result<r2d2::Pool<Client>> {
    let opt = RedisOptions::from_file(path)?;
    redis_client::create_connection_pool(opt)
}

/// 初始化 `MySQL` 连接池.
///
/// # Arguments
///
/// * `path` - 配置文件路径, 包含 `MySQL` 连接信息.
///
/// # Errors
///
/// 如果读取配置文件失败, 或者创建连接池失败, 会返回相应的错误.
pub fn init_mysql(path: &str) -> Result<mysql_async::Pool> {
    let opt = MySQLOptions::from_file(path)?;
    mysql_client::create_connection_pool(opt)
}
