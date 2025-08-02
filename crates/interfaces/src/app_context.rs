//! 整个应用程序中

use anyhow::Result;
use internal_ffi::{init_mqtt_client, init_mysql, init_redis};
use rumqttc::v5::EventLoop;

/// 主要用来创建所有实例, 以及依赖注入.
///
/// AppContext 中的所有实例, 在整个应用程序中共享.
pub(crate) struct AppContext {
    /// mqtt 事件
    pub mqtt_event_loop: Option<EventLoop>,
}

impl AppContext {
    /// 创建 AppContext
    pub(crate) async fn build() -> Result<AppContext> {
        let _mysql_pool = init_mysql("./config/mysql.yaml")?;
        let _redis_pool = init_redis("./config/redis.yaml")?;
        let (_mqttv5client, mqtt_event_loop) = init_mqtt_client("./config/mqtt.yaml").await?;

        Ok(AppContext {
            mqtt_event_loop: Some(mqtt_event_loop),
        })
    }
}
