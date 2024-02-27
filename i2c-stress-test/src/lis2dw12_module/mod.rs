pub mod types;

use linux_embedded_hal::{i2cdev::linux::LinuxI2CError, I2cdev};
use lis2dw12_sensor::LIS2DW12;
use lis2dw12_sensor::constants::*;
use std::{thread::sleep, time::Duration};
use tokio::sync::mpsc;
use types::*;

pub fn setup_component(addr: u8) -> Result<LIS2DW12<I2cdev>, LinuxI2CError>{
    // create I2cdev instance
    let i2c_dev = I2cdev::new("/dev/i2c-5")?;

    // create a accelerometer device
    let mut accelerometer = LIS2DW12::new(i2c_dev, addr);

    // soft reset the device before setup
    accelerometer.soft_reset().unwrap();

    // set full scale selection to 2g
    accelerometer.set_full_scale_selection(FullScaleSelection::MaxAcceleration2g).unwrap();

    // set operation mode to high perfromance
    accelerometer.set_operation_mode(OperationMode::HighPerformance).unwrap();

    // set low power mode to LOW POWER MODE 1
    accelerometer.set_low_power_mode(LowPowerMode::LpMode1).unwrap();

    // set bandwidth filter
    accelerometer.set_bandwidth_filter(BandWidthSelection::OdrDiv10).unwrap();

    // set output data rate
    accelerometer.set_output_data_rate(DataRateConfig::Hp400hzLp200hz).unwrap();

    // read gain/ per-step value/ lsb value from data output
    let gain = accelerometer.calc_out_lsb().unwrap();

    return Ok(accelerometer);
}

pub async fn read_raw_acceleration(addr: u8, interval: u64, data_tx: mpsc::Sender<AccelerometerSensorData>){
    let mut acc = setup_component(addr).unwrap();

    loop{
        let raw_data = acc.read_raw().unwrap();
        // println!("Raw X:{:>8}, Y:{:>8}, Z:{:>8}", raw_data.x_axis, raw_data.y_axis, raw_data.z_axis);

        let _res = data_tx.send(AccelerometerSensorData{
                                                                                x_axis: raw_data.x_axis,
                                                                                y_axis: raw_data.y_axis,
                                                                                z_axis: raw_data.z_axis
        }).await;

        tokio::time::sleep(Duration::from_millis(interval)).await;
    }
}