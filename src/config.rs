use std::{
    fmt::Display,
    fs::File,
    io::{self, Read},
    path::Path,
};

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

#[derive(Deserialize, PartialEq, PartialOrd, Debug)]
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

impl Display for SensorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            SensorType::Temp => String::from("Temp"),
            SensorType::Volt => String::from("Volt"),
            SensorType::Fan => String::from("Fan"),
            SensorType::Current => String::from("Current"),
            SensorType::Power => String::from("Power"),
            SensorType::Clock => String::from("Clock"),
            SensorType::Usage => String::from("Usage"),
            SensorType::Other => String::from("Other"),
        };

        write!(f, "{string}")
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub broker: BrokerConfig,
    pub devices: Vec<Device>,
}

impl Config {
    pub fn load(path: &Path) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut config_str: Config =
            toml::from_str(&contents).expect("Failed to parse config.toml");

        for device in &mut config_str.devices {
            for (i, sensor) in device.sensors.iter_mut().enumerate() {
                sensor.reg_key_name = sensor.sensor_type.to_string() + i.to_string().as_str();
            }
        }

        Ok(config_str)
    }
}
