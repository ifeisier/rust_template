//! 用来创建 MQTT 客户端.

use anyhow::Result;
use internal_shared::yaml::from_yaml_file;
use rumqttc::{
    Error,
    v5::{
        Event,
        mqttbytes::v5::Packet,
        {AsyncClient, MqttOptions, mqttbytes::QoS},
    },
};
use serde::Deserialize;
use std::time::Duration;
use tokio::{sync::mpsc, time::sleep};

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
    /// 从 YAML 文件加载 MQTT 客户端配置
    ///
    /// # 参数
    /// * `path` - YAML 配置文件的路径
    ///
    /// # 返回值
    /// 解析后的 MQTT 客户端配置
    ///
    /// # Errors
    /// 当文件不存在、无法读取或 YAML 格式解析失败时，返回包含相应错误信息的 `anyhow::Error`
    #[allow(dead_code)]
    pub fn from_file(path: &str) -> Result<Self> {
        from_yaml_file(path)
    }
}

/// MQTT v5.0 客户端
pub struct MQTTV5Client;
#[allow(dead_code)]
impl MQTTV5Client {
    /// 连接到 MQTT 服务器并返回异步客户端和事件接收器
    ///
    /// # 参数
    /// * `client_info` - 包含客户端配置信息的 `MqttClientOptions` 结构体
    ///
    /// # Errors
    /// - 连接服务器失败时返回错误
    /// - 初始订阅主题失败时返回错误
    /// - 创建异步通道失败时返回错误
    pub async fn connect(
        client_info: MqttClientOptions,
    ) -> Result<(AsyncClient, mpsc::Receiver<Event>)> {
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

        Ok((client, event_rx))
    }

    /// 判断和返回 v5.0 的 qos
    pub(crate) fn qos(qos: u8) -> Result<QoS> {
        Ok(match qos {
            0 => Ok(QoS::AtMostOnce),
            1 => Ok(QoS::AtLeastOnce),
            2 => Ok(QoS::ExactlyOnce),
            qos => Err(Error::InvalidQoS(qos)),
        }?)
    }
}
