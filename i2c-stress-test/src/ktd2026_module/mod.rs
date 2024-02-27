pub mod types;

use std::time::Duration;

use linux_embedded_hal::{i2cdev::{core::I2CDevice, linux::LinuxI2CError}, I2cdev};

pub fn setup_component() -> I2cdev{
    // create I2cdev instance
    let mut i2c_dev = I2cdev::new("/dev/i2c-5").unwrap();
    i2c_dev.set_slave_address(0x30).unwrap();
    // i2c_dev.write(&[0x04, 0x01]).unwrap();

    i2c_dev
}

pub async fn set_rgb(interval: u64){
    let sensor_address = 0x30;
    let register_address = 0x04;
    let rgb_value = [0x01, 0x04, 0x10, 0x00];

    let mut led_driver = setup_component();
    
    let mut i = 0;
    loop{
        led_driver.write(&[register_address, rgb_value[i]]).unwrap();
        i += 1;

        if i > 3{
            i = 0;
        }

        tokio::time::sleep(Duration::from_millis(interval)).await;
    }    
}