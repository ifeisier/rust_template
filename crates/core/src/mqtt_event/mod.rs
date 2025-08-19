//! 处理 MQTT 事件.

use rumqttc::v5::Event;
use tokio::sync::mpsc;

/// 分发处理 MQTT 事件.
pub fn dispatch_mqtt_event(mut event_loop: mpsc::Receiver<Event>) {
    tokio::spawn(async move {
        while let Some(event) = event_loop.recv().await {
            log::info!("MQTT事件: {event:?}");
        }
        log::warn!("MQTT事件通道已关闭.");
    });
}
