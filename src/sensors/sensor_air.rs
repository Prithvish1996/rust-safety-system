use rand::Rng;
use crate::sensors::{Sensor, SensorError};

pub struct AirQualitySensor {
    failure_rate: f32,
}

impl AirQualitySensor {
    pub fn new(failure_rate: f32) -> Self {
        Self { failure_rate }
    }
}

impl Sensor for AirQualitySensor {
    type Output = i32;

    fn read(&self) -> Result<Self::Output, SensorError> {
        let mut rng = rand::thread_rng();

        if rng.gen_bool(self.failure_rate as f64) {
            return Err(SensorError::Failed);
        }

        Ok(rng.gen_range(0..150))
    }
}