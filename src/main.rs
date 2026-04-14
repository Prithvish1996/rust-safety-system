mod action_layer;
mod model;
mod policy_engine;
mod sensors;
mod state_machine;

use crate::action_layer::ActionLayer;
use crate::model::system_snapshot::SystemSnapshot;
use crate::policy_engine::engine::PolicyEngine;
use crate::sensors::Sensor;
use crate::sensors::sensor_actuator::ActuatorSensor;
use crate::sensors::sensor_air::AirQualitySensor;
use crate::sensors::sensor_temp::TemperatureSensor;
use crate::state_machine::StateMachine;

fn main() {
    let mut state_machine = StateMachine::new(); 
    let mut critical_count = 0;

    loop {
        println!("\n--- Safety System Simulation ---");

        // Sensors
        let temp = TemperatureSensor::new(0.2);
        let air = AirQualitySensor::new(0.1);
        let actuator = ActuatorSensor::new(0.15);

        // Snapshot
        let snapshot = SystemSnapshot {
            temperature: temp.read().unwrap_or(25),
            air_quality: air.read().unwrap_or(50),
            actuator_state: actuator.read().unwrap_or("OFF"),
        };

        // Policy evaluation
        let decision = PolicyEngine::evaluate(&snapshot);

        // State update
        state_machine.update(&decision);

        // Output snapshot
        println!("--- System Snapshot ---");
        println!("Temperature: {}", snapshot.temperature);
        println!("Air Quality: {}", snapshot.air_quality);
        println!("Actuator: {}", snapshot.actuator_state);

        // Output decision
        println!("--- Policy Decision ---");
        match &decision {
            policy_engine::decision::Decision::Safe => {
                println!("SAFE: System is operating normally.")
            }
            policy_engine::decision::Decision::Warning(msg) => {
                println!("WARNING: {}", msg)
            }
            policy_engine::decision::Decision::Critical(msg) => {
                println!("CRITICAL: {}", msg)
            }
        }

        // Output state
        println!("--- System State ---");
        match state_machine.state {
            state_machine::machine::SystemState::Safe => println!("State: SAFE"),
            state_machine::machine::SystemState::Warning => println!("State: WARNING"),
            state_machine::machine::SystemState::Critical => println!("State: CRITICAL"),
        }

    
        ActionLayer::execute(&state_machine.state);

        // Critical handling (debounce logic)
        if let state_machine::machine::SystemState::Critical = state_machine.state {
            critical_count += 1;
            println!("Critical count: {}", critical_count);
        } else {
            critical_count = 0;
        }

        // Shutdown condition
        if critical_count >= 5 {
            ActionLayer::shutdown();
            break;
        }


        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}