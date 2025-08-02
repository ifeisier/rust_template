//! 处理 MQTT 事件.

use rumqttc::v5::Event;
use tokio::sync::mpsc;

/// 分发处理 MQTT 事件.
pub fn dispatch_mqtt_event(mut event_loop: mpsc::Receiver<Event>) {
    tokio::spawn(async move {
        match event_loop.recv().await {
            None => {
                log::warn!("MQTT事件通道已关闭.");
            }
            Some(event) => log::info!("MQTT事件: {:#?}", event),
        }
    });
}
