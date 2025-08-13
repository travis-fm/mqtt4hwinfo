use std::time::Duration;

use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("mqtt4hwinfo", "homeassistant0.isrv.localnet.sh", 1883);
    mqttoptions.set_credentials("test-account", "abc123");
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("office_pc_power_info/active_power", QoS::AtLeastOnce)
        .await
        .unwrap();

    loop {
        let notification = eventloop.poll().await.unwrap();

        if let Event::Incoming(Packet::Publish(publish)) = notification {
            let payload_raw = publish.payload.to_vec();
            let payload = String::from_utf8(publish.payload.to_vec()).unwrap();
            println!("{payload_raw:?} -> {payload:?}");
        }
    }
}
