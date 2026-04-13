use crate::model::system_snapshot::SystemSnapshot;
use crate::policy_engine::decision::Decision;

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn evaluate(snapshot: &SystemSnapshot) -> Decision {
       
        if snapshot.temperature > 98 && snapshot.actuator_state == "OFF" {
            return Decision::Critical(
                "CRITICAL: Overheat with actuator failure".to_string(),
            );
        }

        if snapshot.temperature > 98 {
            return Decision::Critical(
                "CRITICAL: High temperature detected".to_string(),
            );
        }

       
        if snapshot.air_quality > 100 {
            return Decision::Warning(
                "WARNING: Air quality degraded".to_string(),
            );
        }

        if snapshot.actuator_state == "OFF" {
            return Decision::Warning(
                "WARNING: Actuator is OFF".to_string(),
            );
        }

        Decision::Safe
    }
}