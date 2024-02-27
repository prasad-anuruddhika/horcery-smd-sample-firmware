use std::fmt;

#[derive(Debug)]
pub enum Ina219Error{
    I2cReadError,
    // I2cWriteError,
    // I2cReadWriteError,
    // CommunicationError,
    // VoltageReadError,
    // CurrentReadError,
}

// structure used to pass INA219 current sensor data
pub struct CurrentSensorData{
    pub voltage: f32,
    pub current: f32,
    pub power: f32,
    pub shunt_voltage: f32,
}

impl fmt::Display for Ina219Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}