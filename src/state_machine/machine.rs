#[derive(Debug, Clone)]
pub enum SystemState {
    Safe,
    Warning,
    Critical,
}

use crate::policy_engine::decision::Decision;

pub struct StateMachine {
    pub state: SystemState,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            state: SystemState::Safe,
        }
    }

    pub fn update(&mut self, decision: &Decision) {
    use crate::state_machine::SystemState;
    use crate::policy_engine::decision::Decision;

    self.state = match (&self.state, decision) {
        (_, Decision::Critical(_)) => SystemState::Critical,

        (SystemState::Critical, _) => SystemState::Critical,

        (_, Decision::Warning(_)) => SystemState::Warning,

        (_, Decision::Safe) => SystemState::Safe,
    };
}
}