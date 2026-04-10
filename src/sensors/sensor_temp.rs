// Temperature sensor implementation
use rand::Rng;  // For simulating sensor readings and failures

#[derive(Debug)] 
pub enum SensorError {
    Failed,
}

// Simulates a temperature sensor that can fail randomly based on a specified failure rate.
pub struct TemperatureSensor {
    failure_rate: f32,
}

impl TemperatureSensor {
    pub fn new(failure_rate: f32) -> Self {
        Self { failure_rate }
    }

    pub fn read(&self) -> Result<i32, SensorError> {
        let mut rng = rand::thread_rng();

        if rng.gen_bool(self.failure_rate as f64) {
            return Err(SensorError::Failed);
        }

        Ok(rng.gen_range(20..35))
    }
}