//! 整个应用程序的上下文.

use anyhow::Result;
use internal_core::mqtt_event::dispatch_mqtt_event;
use internal_ffi::{init_mqtt_client, init_mysql, init_redis};

/// 主要用来创建所有实例, 以及依赖注入.
///
/// AppContext 中的所有实例, 在整个应用程序中共享.
pub(crate) struct AppContext;

impl AppContext {
    /// 创建 AppContext
    pub(crate) async fn build() -> Result<AppContext> {
        let _mysql_pool = init_mysql("./config/mysql.yaml")?;
        let _redis_pool = init_redis("./config/redis.yaml")?;
        let (_mqtt_client, mqtt_event) = init_mqtt_client("./config/mqtt.yaml").await?;
        dispatch_mqtt_event(mqtt_event);

        Ok(AppContext)
    }
}
