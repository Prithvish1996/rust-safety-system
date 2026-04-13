mod model;
mod policy_engine;
mod sensors;
mod state_machine;

use crate::model::system_snapshot::SystemSnapshot;
use crate::policy_engine::engine::PolicyEngine;
use crate::sensors::Sensor;
use crate::sensors::sensor_actuator::ActuatorSensor;
use crate::sensors::sensor_air::AirQualitySensor;
use crate::sensors::sensor_temp::TemperatureSensor;
use crate::state_machine::StateMachine;

fn main() {
    loop {
        let temp = TemperatureSensor::new(0.2);
        let air = AirQualitySensor::new(0.1);
        let actuator = ActuatorSensor::new(0.15);

        println!("--- Safety System Simulation ---");

        let snapshot = SystemSnapshot {
            temperature: temp.read().unwrap_or(25),
            air_quality: air.read().unwrap_or(50),
            actuator_state: actuator.read().unwrap_or("OFF"),
        };

        let decision = PolicyEngine::evaluate(&snapshot);
        let mut state_machine = StateMachine::new();
        state_machine.update(&decision);

        println!("--- System Snapshot ---");
        println!("Temperature: {}", snapshot.temperature);
        println!("Air Quality: {}", snapshot.air_quality);
        println!("Actuator: {}", snapshot.actuator_state);

        println!("--- Policy Decision ---");
        match decision {
            policy_engine::decision::Decision::Safe => {
                println!("SAFE: System is operating normally.")
            }
            policy_engine::decision::Decision::Warning(msg) => println!("{}", msg),
            policy_engine::decision::Decision::Critical(msg) => println!("{}", msg),
        }

        println!("--- System State ---");
        match state_machine.state {
            state_machine::machine::SystemState::Safe => println!("System State: SAFE"),
            state_machine::machine::SystemState::Warning => println!("System State: WARNING"),
            state_machine::machine::SystemState::Critical => println!("System State: CRITICAL"),
        }

        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
