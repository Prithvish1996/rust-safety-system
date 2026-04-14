use crate::state_machine::SystemState;

pub struct ActionLayer;

impl ActionLayer {
    pub fn execute(state: &SystemState) {
        match state {
            SystemState::Safe => {
                println!("ACTION: System normal");
                println!("→ Monitoring running");
            }

            SystemState::Warning => {
                println!("ACTION: Warning detected");
                println!("→ Increasing monitoring frequency");
                println!("→ Logging event");
            }

            SystemState::Critical => {
                println!("ACTION: CRITICAL CONDITION");
                println!("→ Preparing shutdown...");
                println!("→ Alerting system...");
            }
        }
    }

    // 🔴 Separate shutdown action (important)
    pub fn shutdown() {
        println!("FINAL ACTION: SYSTEM SHUTDOWN");
        println!("→ Actuator OFF");
        println!("→ Sensors stopped");
        println!("→ System is now in safe state");
    }
}