//! 整个应用程序的上下文.

use anyhow::Result;
use internal_core::mqtt_event::MqttEventDispatchContext;
use internal_ffi::{init_mqtt_client, init_mysql, init_redis};
use rumqttc::v5::AsyncClient;

/// 主要用来创建所有实例, 以及依赖注入.
///
/// `AppContext` 中的所有实例, 在整个应用程序中共享.
#[allow(dead_code)]
pub struct AppContext {
    pub mqtt_event_dispatch_context: Option<MqttEventDispatchContext>,
    pub mqtt_client: AsyncClient,
}

impl AppContext {
    /// 创建 `AppContext`
    pub(crate) async fn build() -> Result<Self> {
        let _mysql_pool = init_mysql("./config/mysql.yaml")?;
        let _redis_pool = init_redis("./config/redis.yaml")?;
        let (client, event_loop) = init_mqtt_client("./config/mqtt.yaml").await?;
        let mqtt_event_dispatch_context = Some(MqttEventDispatchContext {
            client: client.clone(),
            event_loop,
        });

        Ok(Self {
            mqtt_event_dispatch_context,
            mqtt_client: client,
        })
    }
}
