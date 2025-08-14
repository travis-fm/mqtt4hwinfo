# MQTT4HWiNFO

Turn MQTT topic subscriptions into HWiNFO sensors!

This Rust crate parses your config data from config.toml, subscribes to MQTT topics representing sensor data, then creates and updates Windows registry keys and values for display inside of HWiNFO.

## Background

I've been using a PowerShell script I wrote a couple of years ago to achieve this, [published here](https://github.com/travis-fm/HASS-2-HwInfo), and so far it's been working *alright*. It'll sometimes randomly quit out on me, or the scheduled task sometimes won't start it, but for the most part it got the job done.

I've always had a couple of sticking issues with the project, though:

* I *hate* PowerShell scripting. It's overly verbose, it's hard to find documentation for different modules, the syntax is just weird to me sometimes...I could go on. I originally wrote it in that to keep dependencies to a minimum, and I've tried embracing it (and have been trying for years), but it's just not happening. Sorry PS lovers, I know it's powerful, but it just ain't clicking for me.

* It was built for Home Assistant, and *only* Home Assistant. Additionally, the original project uses the HA REST API over WebSocket to get sensor info. That's always seemed like a lot of additional overhead just to update a handful of HWiNFO sensors on your local PC. It makes more sense to me to get it directly from the horses mouth via MQTT directly: you can get data from HA *or any other source*, as long as that data gets published to a MQTT topic.

## Requirements

### MQTT

A MQTT broker is required to send and receive sensor data. If you're using Home Assistant, you can use the Mosquitto MQTT add-on to easily set up and configure one, then use automations to set up and update indiviual MQTT topics. See <https://www.home-assistant.io/integrations/mqtt> for configuration details.

MQTT topics should **ONLY** relate to a single sensor, and **ONLY** contain the sensor data for the payload. Wildcards are **NOT** currently supported.

### HWiNFO

Version 6.10 or greater is needed for custom sensor support. See <https://www.hwinfo.com/forum/threads/custom-user-sensors-in-hwinfo.5817/> for overall information on how custom sensors work.

Simple formulas are not currently supported. When/if I do implement them, they'll require at least version 6.22.

### Rust (optional)

Rustup toolchain is required for cargo install method. Visit <https://rustup.rs/> for install.

## Installation

### Install via Cargo

1. Run ```cargo install mqtt4hwinfo``` in PowerShell/Terminal.

2. Copy/save config.toml, and edit to configure for your instance.

3. Run ```mqtt4hwinfo "C:\path\to\config.toml"```.

### Binary Install

1. Download latest version from the releases page.

2. Unzip into a directory of your choice.

3. Configure config.toml for your instance.

4. Run ```./mqtt4hwinfo``` from PowerShell/Terminal within the EXE directory. Optionally you can place config.toml in a separate location and run as ```./mqtt4hwinfo "C:\path\to\config.toml"``` instead.

If everything is configured correctly, you should now see your custom devices/sensors in HWiNFO. If it's already running, you might have to restart HWiNFO after running mqtt4hwinfo for the first time to get the sensors to appear.

## Example: Fully configured config.toml (and what I use on my own machine)

```toml
[broker]
host = "<my MQTT broker IP/FQDN>"
port = 1883
# username/password optional. Remove for anonymous access
username = "<MQTT account>"
password = "<MQTT password>"

# Each device goes into a [[devices]] block, with one or more [[devices.sensors]] blocks below it.
# Each [[devices.sensors]] block should have one of the following sensor_types:
# * Temp
# * Volt
# * Fan
# * Current
# * Power
# * Clock
# * Usage
# * Other
#
# If the sensor_type is "Other", you can specify a "unit", such as "unit = "kWh"". For the other types, HWiNFO will set the default units to display.
[[devices]]
display_name = "PC Power Usage"

[[devices.sensors]]
display_name = "Active Power"
sensor_type = "Power"
mqtt_topic = "office_pc_power_info/active_power"

[[devices.sensors]]
display_name = "RMS Current"
sensor_type = "Current"
mqtt_topic = "office_pc_power_info/rms_current"

[[devices.sensors]]
display_name = "RMS Voltage"
sensor_type = "Volt"
mqtt_topic = "office_pc_power_info/rms_voltage"

[[devices.sensors]]
display_name = "Power Factor"
sensor_type = "Usage"
mqtt_topic = "office_pc_power_info/power_factor"

[[devices.sensors]]
display_name = "Total Delivered"
sensor_type = "Other"
unit = "kWh"
mqtt_topic = "office_pc_power_info/total_delivered"

[[devices.sensors]]
display_name = "AC Frequency"
sensor_type = "Clock"
mqtt_topic = "office_pc_power_info/ac_frequency"

[[devices.sensors]]
display_name = "Instantaneous Demand"
sensor_type = "Power"
mqtt_topic = "office_pc_power_info/instant_demand"

# To add another device, simply add another [[devices]] block, with another set of [[devices.sensors]]:
#
# [[devices]]
# display_name = "My Next Probe"
#
# [[devices.sensors]]
# display_name = "My Next Temp Sensor"
# sensor_type = "Temp"
# mqtt_topic = "office_temperature_probe/actual_temperature"
#
# [[devices.sensors]]
# display_name = "My Next Humidity Sensor"
# sensor_type = "Other"
# unit = "%"
# mqtt_topic = "office_temperature_probe/actual_humidity" 

```
