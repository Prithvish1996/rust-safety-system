// Temperature sensor implementation
use rand::Rng;  // For simulating sensor readings and failures
use crate::sensors::{Sensor, SensorError}; // Custom error type for sensor failures

// Simulates a temperature sensor that can fail randomly based on a specified failure rate.
pub struct TemperatureSensor {
    failure_rate: f32,
}

impl TemperatureSensor {
    pub fn new(failure_rate: f32) -> Self {
        Self { failure_rate }
    }
}

impl Sensor for TemperatureSensor {
    type Output = i32;

    fn read(&self) -> Result<Self::Output, SensorError> {
        let mut rng = rand::thread_rng();

        if rng.gen_bool(self.failure_rate as f64) {
            return Err(SensorError::Failed);
        }

        Ok(rng.gen_range(20..120))
    }
}