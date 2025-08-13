mod config;

use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS, SubscribeFilter};
use std::time::Duration;

use crate::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load();

    let mut mqttoptions = MqttOptions::new(
        "mqtt4hwinfo",
        config.broker.host,
        config.broker.port.unwrap_or(1883),
    );

    if let Some(user) = config.broker.username
        && let Some(pass) = config.broker.password
    {
        mqttoptions.set_credentials(user, pass);
    }
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let mut topics = vec![];
    for device in config.devices {
        for sensor in device.sensors {
            topics.push(SubscribeFilter {
                path: sensor.mqtt_topic,
                qos: QoS::AtLeastOnce,
            });
        }
    }

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe_many(topics).await.unwrap();

    loop {
        let notification = eventloop.poll().await.unwrap();

        if let Event::Incoming(Packet::Publish(publish)) = notification {
            let topic = publish.topic;
            let payload = String::from_utf8(publish.payload.to_vec()).unwrap();
            println!("{topic:?} -> {payload:?}");
        }
    }
}
