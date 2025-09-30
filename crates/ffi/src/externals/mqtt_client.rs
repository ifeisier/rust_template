//! 这个模块只提供了 MQTT 的客户端实现, 发送消息的简单方法.

use anyhow::Result;
use rumqttc::Error;
use rumqttc::v5::Event;
use rumqttc::v5::mqttbytes::v5::Packet;
use rumqttc::v5::{AsyncClient, MqttOptions, mqttbytes::QoS};
use serde::Deserialize;
use std::time::Duration;
use tokio::{sync::mpsc, time::sleep};
use xx_toolkit::yaml::from_yaml_file;

/// MQTT 客户端信息
#[derive(Debug, Deserialize)]
pub struct MqttClientOptions {
    /// 客户端 ID
    pub id: String,
    /// 服务器地址
    pub host: String,
    /// 服务器端口
    pub port: u16,
    /// 用户名
    pub user_name: String,
    /// 密码
    pub pass_word: String,
    /// 有界异步通道的容量
    pub channel_cap: usize,
    /// 要订阅的主题
    ///
    /// 订阅主题 QOS:0, 由程序控制是否接收消息
    pub subscribes: Vec<String>,
}
impl MqttClientOptions {
    /// 从文件加载配置.
    #[allow(dead_code)]
    pub fn from_file(path: &str) -> Result<Self> {
        from_yaml_file(path)
    }
}

/// MQTT v5.0 客户端
#[derive(Clone, Debug)]
pub struct MQTTV5Client {
    client: AsyncClient,
}
impl MQTTV5Client {
    /// 创建新的 v5.0 客户端
    ///
    /// 返回 (客户端实例, `EventLoop`)
    #[allow(dead_code)]
    pub async fn new(client_info: MqttClientOptions) -> Result<(Self, mpsc::Receiver<Event>)> {
        let mut options = MqttOptions::new(client_info.id, client_info.host, client_info.port);
        options.set_keep_alive(Duration::from_secs(10));
        options.set_clean_start(true);
        options.set_connection_timeout(30);
        options.set_max_packet_size(Some(1_048_576)); // 1048576Byte = 1MB
        options.set_credentials(client_info.user_name, client_info.pass_word);

        let (client, mut event_loop) = AsyncClient::new(options, client_info.channel_cap);
        let restore_subs = client_info.subscribes.clone();
        let restore_client = client.clone();
        for ele in client_info.subscribes {
            client.subscribe(ele, QoS::AtLeastOnce).await?;
        }

        let (tx, event_rx) = mpsc::channel::<Event>(client_info.channel_cap);
        tokio::spawn(async move {
            loop {
                match event_loop.poll().await {
                    Ok(event) => {
                        if let Event::Incoming(Packet::ConnAck(_ack)) = &event {
                            log::debug!("MQTT 已连接, 开始恢复订阅.");
                            for t in &restore_subs {
                                if let Err(e) = restore_client.subscribe(t, QoS::AtLeastOnce).await
                                {
                                    log::error!("重连后订阅 {t} 失败: {e:?}");
                                }
                            }
                        }

                        if let Err(e) = tx.send(event).await {
                            log::error!("将MQTT事件发送到通道错误:{e:?}");
                        }
                    }
                    Err(e) => {
                        log::error!("接收MQTT事件错误:{e:?}");
                        sleep(Duration::from_secs(10)).await;
                    }
                }
            }
        });

        Ok((Self { client }, event_rx))
    }

    /// 发送 MQTT 消息
    #[allow(dead_code)]
    pub fn publish(&self, topic: &str, qos: u8, payload: Vec<u8>) -> Result<()> {
        let qos = qos_v5(qos)?;

        // publish 和 try_publish 的区别
        // publish: 如果有界队列满了, 那么就会阻塞
        // try_publish: 如果有界队列满了, 那么就会返回错误
        self.client.try_publish(topic, qos, false, payload)?;
        Ok(())
    }

    /// 发送 MQTT 保留消息
    #[allow(dead_code)]
    pub fn publish_retain(&self, topic: &str, qos: u8, payload: Vec<u8>) -> Result<()> {
        let qos = qos_v5(qos)?;
        self.client.try_publish(topic, qos, true, payload)?;
        Ok(())
    }

    /// 订阅指定的 MQTT 主题
    #[allow(dead_code)]
    pub fn subscribe(&self, topic: &str, qos: u8) -> Result<()> {
        let qos = qos_v5(qos)?;
        self.client.try_subscribe(topic, qos)?;
        Ok(())
    }

    /// 取消订阅指定的 MQTT 主题
    #[allow(dead_code)]
    pub fn unsubscribe(&self, topic: &str) -> Result<()> {
        self.client.try_unsubscribe(topic)?;
        Ok(())
    }

    // 获取完整的 MQTT 客户端
    // #[allow(dead_code)]
    // pub fn get_client(&self) -> AsyncClient {
    //     self.client.clone()
    // }
}

/// 判断和返回 v5.0 的 qos
fn qos_v5(qos: u8) -> Result<QoS> {
    Ok(match qos {
        0 => Ok(QoS::AtMostOnce),
        1 => Ok(QoS::AtLeastOnce),
        2 => Ok(QoS::ExactlyOnce),
        qos => Err(Error::InvalidQoS(qos)),
    }?)
}
