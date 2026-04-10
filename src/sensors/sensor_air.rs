use rand::Rng;

#[derive(Debug)]
pub enum SensorError {
    Failed,
}

pub struct AirQualitySensor {
    failure_rate: f32,
}

impl AirQualitySensor {
    pub fn new(failure_rate: f32) -> Self {
        Self { failure_rate }
    }

    pub fn read(&self) -> Result<i32, SensorError> {
        let mut rng = rand::thread_rng();

        if rng.gen_bool(self.failure_rate as f64) {
            return Err(SensorError::Failed);
        }

        Ok(rng.gen_range(0..150))
    }
}