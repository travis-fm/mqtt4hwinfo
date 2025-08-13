use std::time::Duration;

use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS, SubscribeFilter};

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("mqtt4hwinfo", "homeassistant0.isrv.localnet.sh", 1883);
    mqttoptions.set_credentials("test-account", "abc123");
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let topics = vec![
        SubscribeFilter::new(
            "office_pc_power_info/rms_current".to_string(),
            QoS::AtLeastOnce,
        ),
        SubscribeFilter::new(
            "office_pc_power_info/rms_voltage".to_string(),
            QoS::AtLeastOnce,
        ),
        SubscribeFilter::new(
            "office_pc_power_info/active_power".to_string(),
            QoS::AtLeastOnce,
        ),
        SubscribeFilter::new(
            "office_pc_power_info/power_factor".to_string(),
            QoS::AtLeastOnce,
        ),
        SubscribeFilter::new(
            "office_pc_power_info/total_delivered".to_string(),
            QoS::AtLeastOnce,
        ),
        SubscribeFilter::new(
            "office_pc_power_info/ac_frequency".to_string(),
            QoS::AtLeastOnce,
        ),
        SubscribeFilter::new(
            "office_pc_power_info/instant_demand".to_string(),
            QoS::AtLeastOnce,
        ),
    ];

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
