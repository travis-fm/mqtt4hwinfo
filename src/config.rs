use serde::Deserialize;

#[derive(Deserialize)]
pub struct BrokerConfig {
    pub host: String,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct DeviceConfig {
    pub name: String,
    pub sensors: Vec<SensorConfig>,
}

#[derive(Deserialize)]
pub struct SensorConfig {
    pub sensor_name: String,
    pub sensor_type: SensorType,
    pub mqtt_topic: String,
}

#[derive(Deserialize)]
pub enum SensorType {
    Temperature,
    Voltage,
    Fan,
    Current,
    Power,
    Clock,
    Usage,
    Other,
}

#[derive(Deserialize)]
pub struct Config {
    pub broker: BrokerConfig,
    pub devices: Vec<DeviceConfig>,
}

impl Config {
    pub fn load() -> Self {
        let config_str: Config =
            toml::from_str(include_str!("../config.toml")).expect("Failed to parse config.toml");

        config_str
    }
}
