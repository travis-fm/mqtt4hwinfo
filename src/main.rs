mod config;

use anyhow::{Context, Result};
use rumqttc::{AsyncClient, ConnectReturnCode, Event, MqttOptions, Packet, QoS, SubscribeFilter};
use std::env;
use std::io::{self};
use std::path::Path;
use std::time::Duration;
use winreg::{RegKey, enums::HKEY_CURRENT_USER};

use crate::config::{Config, Sensor};

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let config_path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new("./config.toml")
    };

    let config = Config::load(config_path)?;

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
    for device in &config.devices {
        for sensor in &device.sensors {
            topics.push(SubscribeFilter {
                path: sensor.mqtt_topic.clone(),
                qos: QoS::AtLeastOnce,
            });
        }
    }

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::ConnAck(connack)))
                if connack.code == ConnectReturnCode::Success =>
            {
                client
                    .subscribe_many(topics.clone())
                    .await
                    .context("Error while subscribing to sensor topics")?;
            }
            Ok(Event::Incoming(Packet::Publish(packet))) => {
                let topic = packet.topic;
                let payload = String::from_utf8(packet.payload.to_vec()).unwrap();
                if let Some(device) = config
                    .devices
                    .iter()
                    .find(|d| d.sensors.iter().any(|s| s.mqtt_topic == topic))
                {
                    if let Some(sensor) = device.sensors.iter().find(|s| s.mqtt_topic == topic) {
                        update_sensor(&device.display_name, sensor, &payload)?;
                    }
                }
                println!("{topic:?} -> {payload:?}");
            }
            _ => {}
        }
    }

    Ok(())
}

fn update_sensor(device_name: &str, sensor: &Sensor, value: &str) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let sensor_key_path = format!(
        "Software\\HWiNFO64\\Sensors\\Custom\\{device_name}\\{0}",
        sensor.reg_key_name
    );
    let (sensor_key, _) = hkcu.create_subkey(sensor_key_path)?;
    sensor_key.set_value("Name", &sensor.display_name)?;
    sensor_key.set_value("Value", &value)?;

    if let Some(unit) = &sensor.unit {
        sensor_key.set_value("Unit", unit)?;
    }

    Ok(())
}
