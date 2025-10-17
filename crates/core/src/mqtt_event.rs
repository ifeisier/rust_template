//! 处理 MQTT 事件.

use rumqttc::v5::{AsyncClient, Event, Event::Incoming, mqttbytes::v5};
use std::time::Duration;
use tokio::{sync::mpsc, time::sleep};

/// MQTT 事件分发上下文.
pub struct MqttEventDispatchContext {
    /// MQTT 客户端.
    pub client: AsyncClient,
    /// MQTT 事件循环接收器.
    pub event_loop: mpsc::Receiver<Event>,
}

/// 分发处理 MQTT 事件.
pub fn dispatch_mqtt_events(mqtt_event_dispatch_context: MqttEventDispatchContext) {
    let mut event_loop = mqtt_event_dispatch_context.event_loop;
    tokio::spawn(async move {
        loop {
            let Some(event) = event_loop.recv().await else {
                sleep(Duration::from_secs(1)).await;
                continue;
            };

            let Some(event) = get_publish_value(event) else {
                continue;
            };
            log::debug!("收到原始MQTT#Publish事件: {event:?}");

            // 调用业务逻辑处理.
        }
    });
}

/// 获取 publish 事件值
pub(crate) fn get_publish_value(event: Event) -> Option<v5::Publish> {
    if let Incoming(v5::Packet::Publish(v)) = event {
        Some(v)
    } else {
        None
    }
}
