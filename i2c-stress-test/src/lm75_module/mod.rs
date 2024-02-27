pub mod types;

use std::time::Duration;

use lm75::{Lm75, Address};
use linux_embedded_hal::{i2cdev::linux::LinuxI2CError, I2cdev};
use tokio::sync::mpsc;
use lm75::ic::Lm75 as Lm75_maker;
use self::types::*;

// method for setup LM75 temperature sensor
pub fn setup_component(addr: u8) -> Result<Lm75<I2cdev, Lm75_maker>, LinuxI2CError>{
    // create I2cdev instance
    let i2c_dev = I2cdev::new("/dev/i2c-5")?;

    // create a LM75 instance
    let temperature_sensor = Lm75::new(i2c_dev, Address::from(addr));

    return Ok(temperature_sensor);
}

pub async fn read_temperature(addr: u8, interval: u64, sensor_id: u8, data_tx: mpsc::Sender<TemperatureSensorData>){
    let mut ts = setup_component(addr).unwrap();

    loop{
        let temperature = ts.read_temperature().unwrap();
        let _res = data_tx.send(TemperatureSensorData{sensor_id, temperature}).await;

        tokio::time::sleep(Duration::from_millis(interval)).await;
    }
}