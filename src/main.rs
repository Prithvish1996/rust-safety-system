mod sensors;
mod model;

use crate::sensors::Sensor;
use crate::model::system_snapshot::SystemSnapshot;
use crate::sensors::sensor_temp::TemperatureSensor;
use crate::sensors::sensor_air::AirQualitySensor;
use crate::sensors::sensor_actuator::ActuatorSensor;

fn main() {
    let temp = TemperatureSensor::new(0.2);
    let air = AirQualitySensor::new(0.1);
    let actuator = ActuatorSensor::new(0.15);

    println!("--- Safety System Simulation ---");

    let snapshot = SystemSnapshot {
    temperature: temp.read().unwrap_or(25),
    air_quality: air.read().unwrap_or(50),
    actuator_state: actuator.read().unwrap_or("OFF"),
};

   println!("--- System Snapshot ---");
    println!("Temperature: {}", snapshot.temperature);
    println!("Air Quality: {}", snapshot.air_quality);
    println!("Actuator: {}", snapshot.actuator_state);
}

