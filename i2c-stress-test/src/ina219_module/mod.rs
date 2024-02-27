pub mod types;

use std::time::Duration;

use tokio::sync::mpsc;
use ina219_sensor::*;
use linux_embedded_hal::{i2cdev::linux::LinuxI2CError, I2cdev};
use self::types::*;

// method for setup INA219 current sensor
pub fn setup_component(addr: u8) -> Result<INA219<I2cdev>, LinuxI2CError>{
    // create I2cdev instance
    let i2c_dev = I2cdev::new("/dev/i2c-5")?;

    // create a instance with INA219
    let mut current_sensor = INA219::new(i2c_dev, addr);
    
    // configure the current sensor
    current_sensor.configure(RANGE_16V, 
        GAIN_1_40MV, 
        ADC_4SAMP, 
        ADC_16SAMP, 
        SHUNT_BUS_VOLT_CONTINUOUS, 
        0.01)?;

    // calibrate the current sensor
    current_sensor.calibrate(SHUNT_VOLT_40MV)?;

    return  Ok(current_sensor);
}

// method for read current reading from the sensor
// need to provide a pointer for INA219 instance
pub fn _read_current(ptr_current_sesnor: &mut INA219<I2cdev>) -> Result<f32, Ina219Error>{
    // read current from sensor
    let current = match ptr_current_sesnor.current() {
        Ok(amp) => amp,
        Err(_) => {
            return Err(Ina219Error::I2cReadError);
        }
    };

    // return current value
    Ok(current)
}

pub async fn read_power_statics(addr: u8, interval: u64, data_tx: mpsc::Sender<CurrentSensorData>){
    let mut cs = setup_component(addr).unwrap();
    
    loop {
        let voltage = cs.voltage().unwrap();
        let current = cs.current().unwrap();
        let power = cs.power().unwrap();
        let shunt_voltage = cs.shunt_voltage().unwrap();

        let _res = data_tx.send(CurrentSensorData{voltage, current, power, shunt_voltage}).await;
        
        tokio::time::sleep(Duration::from_millis(interval)).await;
    }
}

