//! 处理 MQTT 事件.

use rumqttc::v5::Event;
use rumqttc::v5::Event::Incoming;
use rumqttc::v5::mqttbytes::v5;
use tokio::sync::mpsc;

/// 分发处理 MQTT 事件.
pub fn dispatch_mqtt_event(mut event_loop: mpsc::Receiver<Event>) {
    tokio::spawn(async move {
        while let Some(event) = event_loop.recv().await {
            let Some(event) = get_publish_value(event) else {
                continue;
            };
            log::info!("MQTT事件: {event:?}");
        }
        log::warn!("MQTT事件通道已关闭.");
    });
}

/// 获取 publish 事件值
pub fn get_publish_value(event: Event) -> Option<v5::Publish> {
    if let Incoming(v5::Packet::Publish(v)) = event {
        Some(v)
    } else {
        None
    }
}
