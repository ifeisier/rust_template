#![warn(missing_docs)]

//! 调用外部系统 (FFI, gRPC, 数据库访问, HTTP 客户端等)

use crate::externals::mqtt_client::MQTTV5Client;
use anyhow::Result;
use redis::Client;
use rumqttc::v5::Event;
use tokio::sync::mpsc;

pub(crate) mod externals;
pub mod impls;

/// 初始化 Mqtt 客户端
pub async fn init_mqtt_client(path: &str) -> Result<(MQTTV5Client, mpsc::Receiver<Event>)> {
    let opt = externals::mqtt_client::MqttClientOptions::from_file(path)?;
    MQTTV5Client::new(opt).await
}

/// 初始化 redis 连接池
pub fn init_redis(path: &str) -> Result<r2d2::Pool<Client>> {
    let opt = externals::redis::RedisOptions::from_file(path)?;
    externals::redis::create_connection_pool(opt)
}

/// 初始化 mysql 连接池
pub fn init_mysql(path: &str) -> Result<mysql::Pool> {
    let opt = externals::mysql::MySQLOptions::from_file(path)?;
    externals::mysql::create_connection_pool(opt)
}
