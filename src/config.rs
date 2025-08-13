use serde::Deserialize;

#[derive(Deserialize)]
pub struct BrokerConfig {
    pub host: String,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename = "devices")]
pub struct Device {
    pub display_name: String,
    pub sensors: Vec<Sensor>,
}

#[derive(Deserialize)]
#[serde(rename = "sensors")]
pub struct Sensor {
    pub display_name: String,
    #[serde(default)]
    pub reg_key_name: String,
    pub sensor_type: SensorType,
    pub mqtt_topic: String,
    pub unit: Option<String>,
}

#[derive(Deserialize, PartialEq, PartialOrd)]
pub enum SensorType {
    Temp,
    Volt,
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
            SensorType::Temp => String::from("Temp"),
            SensorType::Volt => String::from("Volt"),
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
    pub devices: Vec<Device>,
}

impl Config {
    pub fn load() -> Self {
        let mut config_str: Config =
            toml::from_str(include_str!("../config.toml")).expect("Failed to parse config.toml");

        for device in &mut config_str.devices {
            for (i, sensor) in device.sensors.iter_mut().enumerate() {
                sensor.reg_key_name =
                    sensor.sensor_type.to_key_base_string() + i.to_string().as_str();
            }
        }

        config_str
    }
}
