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
    pub sensor_key_name: String,
    pub sensor_type: SensorType,
    pub mqtt_topic: String,
}

#[derive(Deserialize, PartialEq, PartialOrd)]
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

impl SensorType {
    /// Converts SensorType value into string used for the key base name in the registry.
    ///
    /// See https://www.hwinfo.com/forum/threads/custom-user-sensors-in-hwinfo.5817/
    fn to_key_base_string(&self) -> String {
        match self {
            SensorType::Temperature => String::from("Temp"),
            SensorType::Voltage => String::from("Volt"),
            SensorType::Fan => String::from("Fan"),
            SensorType::Current => String::from("Current"),
            SensorType::Power => String::from("Power"),
            SensorType::Clock => String::from("Clock"),
            SensorType::Usage => String::from("Usage"),
            SensorType::Other => String::from("Other"),
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub broker: BrokerConfig,
    pub devices: Vec<DeviceConfig>,
}

impl Config {
    pub fn load() -> Self {
        let mut config_str: Config =
            toml::from_str(include_str!("../config.toml")).expect("Failed to parse config.toml");

        for device in &mut config_str.devices {
            for (i, sensor) in device.sensors.iter_mut().enumerate() {
                sensor.sensor_key_name =
                    sensor.sensor_type.to_key_base_string() + i.to_string().as_str();
            }
        }

        config_str
    }
}
