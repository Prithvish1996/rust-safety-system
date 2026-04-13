pub mod error;
pub mod sensor_trait;
pub mod sensor_temp;
pub mod sensor_air;
pub mod sensor_actuator;

pub use error::SensorError;
pub use sensor_trait::Sensor;