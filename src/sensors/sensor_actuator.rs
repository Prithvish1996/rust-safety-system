use rand::Rng;

use crate::sensors::{Sensor, SensorError};

pub struct ActuatorSensor {
    failure_rate: f32,
}

impl ActuatorSensor {
    pub fn new(failure_rate: f32) -> Self {
        Self { failure_rate }
    }
}

impl Sensor for ActuatorSensor {
    type Output = &'static str;

    fn read(&self) -> Result<Self::Output, SensorError> {
        let mut rng = rand::thread_rng();

        if rng.gen_bool(self.failure_rate as f64) {
            return Err(SensorError::Failed);
        }

        let states = ["ON", "OFF", "RUNNING"];
        Ok(states[rng.gen_range(0..3)])
    }
}