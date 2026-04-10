mod sensors;

use crate::sensors::sensor_temp::TemperatureSensor;
use crate::sensors::sensor_air::AirQualitySensor;
use crate::sensors::sensor_actuator::ActuatorSensor;

fn main() {
    let temp = TemperatureSensor::new(0.2);
    let air = AirQualitySensor::new(0.1);
    let actuator = ActuatorSensor::new(0.15);

    println!("--- Safety System Simulation ---");

    println!("Temperature: {:?}", temp.read());
    println!("Air Quality: {:?}", air.read());
    println!("Actuator: {:?}", actuator.read());
}