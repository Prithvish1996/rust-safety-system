use crate::sensors::SensorError;

pub trait Sensor {
    type Output;

    fn read(&self) -> Result<Self::Output, SensorError>;
}