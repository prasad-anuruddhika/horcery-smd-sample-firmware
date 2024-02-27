pub mod types;

use std::time::Duration;

use linux_embedded_hal::{i2cdev::linux::LinuxI2CError, I2cdev};
use veml7700::{self, Veml7700};
use types::*;
use tokio::sync::mpsc;

pub fn setup_component() -> Result<Veml7700<I2cdev>, LinuxI2CError>{
    println!("Setup");
    // create I2cdev instance
    let i2c_dev = I2cdev::new("/dev/i2c-5")?;

    // create a light sensor device
    let mut light_sensor = Veml7700::new(i2c_dev);

    return Ok(light_sensor);
}

pub async fn read_light_sensor(interval: u64, data_tx: mpsc::Sender<LightSensorData>){
    println!("t1");
    let mut veml7700_device = setup_component().unwrap();

    loop{
        let lux = veml7700_device.read_lux().unwrap();
        let white = veml7700_device.read_white().unwrap();
        let raw = veml7700_device.read_raw().unwrap();

        let _res = data_tx.send(LightSensorData {
                                                                            lux, 
                                                                            white, 
                                                                            raw
        }).await;
        
        tokio::time::sleep(Duration::from_millis(interval)).await;
    }
}