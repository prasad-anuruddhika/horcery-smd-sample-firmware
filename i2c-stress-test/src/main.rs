/*
This is not a production environment code
*/

use once_cell::sync::OnceCell;
use tokio::sync::mpsc;

mod ina219_module;
mod lm75_module;
mod bme680_module;
mod lis2dw12_module;
mod veml7700_module;
mod ktd2026_module;
mod constants;

use bme680_module::types::*;
use constants::*;
use chrono::Local;

use std::ffi::{c_int, c_float, c_longlong, c_uchar};
// create a staic variable with type of OnceCell to hold the BME680 Sender 
static CHANNEL_SENDER: OnceCell<mpsc::Sender<Bme680SensorData>> = OnceCell::new();

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // create a new mpsc channel for receive data from current sensor
    let (tx_ina219, mut rx_ina219) = mpsc::channel(DEFAULT_CHANNEL_BUFFER);

    // LM75 temperature sensors
    let (tx_lm75, mut rx_lm75) = mpsc::channel(DEFAULT_CHANNEL_BUFFER);

    // clone a sender from tx_lm75
    let tx2_lm75 = tx_lm75.clone();

    // create a new mpsc channel for receive data from bme sensor
    let (tx_bme680, mut rx_bme680) = mpsc::channel(DEFAULT_CHANNEL_BUFFER);

    // create a new mpsc channel for receive data from accelerometer
    let (tx_lis2dw12, mut rx_lis2dw12) = mpsc::channel(DEFAULT_CHANNEL_BUFFER);

    // create a new mpsc channel for receive data from light sensor
    let (tx_veml7700, mut rx_veml7700) = mpsc::channel(DEFAULT_CHANNEL_BUFFER);

    // create a thread to handle under current sensor
    let _current_sensor_handle = tokio::spawn(async move {
        // intialize the current reading loop
        ina219_module::read_power_statics(0x40, 
                        INTERVAL_CURRENT_READ, 
                        tx_ina219).await;
    });

    // thread for handle [u8] LM75 Sensor
    let _temperature_sensor_u8 = tokio::spawn(async move {
        lm75_module::read_temperature(0x4A, 
                                    INTERVAL_TEMPERATURE_1_READ, 
                                    1, 
                                    tx_lm75).await;
    });

    // thread for handle [u15] LM75 Sensor
    let _temperature_sensor_u15 = tokio::spawn(async move {
        lm75_module::read_temperature(0x48, 
                                    INTERVAL_TEMPERATURE_2_READ, 
                                    2, 
                                    tx2_lm75).await;
    });

    let _accelerometer_handle = tokio::spawn(async move{
        lis2dw12_module::read_raw_acceleration(0x19, 
                                                INTERVAL_ACCELEROMETER_READ, 
                                                tx_lis2dw12).await;
    });

    let _light_sensor_handle = tokio::spawn(async move{
        veml7700_module::read_light_sensor( INTERVAL_LIGHT_READ, 
                                            tx_veml7700).await;
    });

    let _rgb_driver_handle = tokio::spawn(async move{
        ktd2026_module::set_rgb(INTERVAL_RGB_SET).await;
    });

    // let _bme_sensor_handle
    let _bme_sensor_handle = tokio::spawn(async move { 
        
        // assign BME680 sender for global sender
        let _res = CHANNEL_SENDER.get_or_init(|| {
            tx_bme680
        });

        let mut res = 0;
        let mut has_fault = false;
        unsafe{
            // open the I2C port through C method
            res = i2c_open();
        }

        if res != 0{
            has_fault = true;
        }

        if !has_fault{
            unsafe{
                // set the I2C Address
                res = i2c_set_address(0x77);
            }
        }

        if res != 0{
            has_fault = true;
        }

        if !has_fault{
            // initialize the bsec library
            let bsec_init_result: i32 = unsafe {
                bsec_initializer(BSEC_SAMPLE_RATE_LP, BSEC_TEMPERATURE_OFFSET)
            };

            println!("BSEC Init Error Code: {}", bsec_init_result);

            if bsec_init_result != 0 {
                has_fault = true;
            }
        }
        
        if !has_fault{
            // run Bsec Loop 
            unsafe{
                bsec_loop(bsec_output_ready_callback);
            }
        }
        
    });

    loop{
        tokio::select! {
            Some(received_ina219_data) = rx_ina219.recv() => {
                print_timestamp();
                println!("V:{:>4.2} C:{:>7.2} SV:{:>5.2} P:{:>7.2}",
                        received_ina219_data.voltage, 
                        received_ina219_data.current,
                        received_ina219_data.shunt_voltage,
                        received_ina219_data.power
                    );
                // println!("Voltage: {:.03}V  Current: {:.03}mA Shunt Voltage: {:.03}mV Power: {:.03}mW", 
                //     received_ina219_data.voltage, 
                //     received_ina219_data.current, 
                //     received_ina219_data.shunt_voltage, 
                //     received_ina219_data.power);
            }

            Some(received_lm75_data) = rx_lm75.recv() => {
                print_timestamp();
                print!("{:>35}", " ");
                if received_lm75_data.sensor_id == 1{
                    print!("{:>1}", " ");
                }
                else{
                    print!("{:>10}", " ");
                }
                println!("T{:>1}:{:>5.2}",
                    received_lm75_data.sensor_id,
                    received_lm75_data.temperature
                );
                // println!("Temperature data from sensor {} is {:.02}", 
                //     received_lm75_data.sensor_id, 
                //     received_lm75_data.temperature);
            }

            Some(received_lis2dw12_data) = rx_lis2dw12.recv() => {
                // println!("Accelerometer data X: {:>6} Y: {:>6} Z: {:>6}",
                //     received_lis2dw12_data.x_axis,
                //     received_lis2dw12_data.y_axis,
                //     received_lis2dw12_data.z_axis
                // );
                print_timestamp();
                print!("{:>54}", " ");
                println!("X:{:>5} Y:{:>5} Z:{:>5}",
                    received_lis2dw12_data.x_axis,
                    received_lis2dw12_data.y_axis,
                    received_lis2dw12_data.z_axis,
                );
            }

            Some(received_veml7700_data) = rx_veml7700.recv() => {
                print_timestamp();
                print!("{:>78}", " ");

                println!("Lx:{:>6.2} Wh:{:>5} Rw:{:>5}",
                    received_veml7700_data.lux,
                    received_veml7700_data.white,
                    received_veml7700_data.raw,
                );
                // println!("Light Sensor Lux: {:0.03} White: {}, Raw: {}",
                //     received_veml7700_data.lux,
                //     received_veml7700_data.white,
                //     received_veml7700_data.raw
                // );
            }

            // receive from BME680 channel
            Some(received_bme680_data) = rx_bme680.recv() => {
                print_timestamp();
                print!("{:>106}", " ");

                println!("IAQ({:>1}):{:>6.2} T:{:>5.2} H:{:>5.2} P:{:>7.2} CO2:{:>7.3} VOC:{:>7.3}",
                    received_bme680_data.iaq_accuracy,
                    received_bme680_data.iaq,
                    received_bme680_data.temperature,
                    received_bme680_data.humidity,
                    received_bme680_data.pressure/100.0,
                    received_bme680_data.co2_equivalent,
                    received_bme680_data.breath_voc_equivalent
                );

                // print!("BME [IAQ ({})]: {:.2}", received_bme680_data.iaq_accuracy, received_bme680_data.iaq);
                // print!(", [T degC]: {:.2}, [H %rH]: {:.2}, [P hPa]: {:.2}", 
                //         received_bme680_data.temperature, received_bme680_data.humidity, 
                //         received_bme680_data.pressure/100.0);
                // print!(", [G Ohms]: {}", received_bme680_data.gas);
                // print!(", [eCO2 ppm]: {:.8}", received_bme680_data.co2_equivalent);
                // println!(", [bVOCe ppm]: {:.8}", received_bme680_data.breath_voc_equivalent);
            }
        }
    }

}


fn print_timestamp(){
    // Get the current local time
    let datetime_local = Local::now();

    // Format the local datetime as hours:minutes:seconds
    let formatted_time = datetime_local.format("%H:%M:%S:%3f").to_string();

    // Print the formatted time
    print!("{:>12}{:>2}", formatted_time, " ");
}
// external C functions
extern "C"{
    // this method used to open I2C port with particular bus
    fn i2c_open() -> i32;

    // this method used to set device I2C address
    fn i2c_set_address(address: c_int) -> i32;

    // method for initialize BSec Library
    fn bsec_initializer(sampling_mode: c_float,
        temperature_offset: c_float) -> c_int;
    
    // method for run the loop of BSec Library
    fn bsec_loop(
        output_ready : extern "C" fn(
            c_longlong,
            c_float,
            c_uchar,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            c_float,
            BsecLibraryReturn,
            c_float,
            c_float,
            c_float
        )
    );
}

// bsec library callback method
extern "C" fn bsec_output_ready_callback(
    _timestamp: c_longlong,
    iaq: c_float,
    iaq_accuracy: c_uchar,
    temperature: c_float,
    humidity: c_float,
    pressure: c_float,
    _raw_temperature: c_float,
    _raw_humidity: c_float,
    gas: c_float,
    bsec_status: BsecLibraryReturn,
    _static_iaq: c_float,
    co2_equivalent: c_float,
    breath_voc_equivalent: c_float
){

    match bsec_status {
        BsecLibraryReturn::BsecOk => {
            // println!("[Rust][Callback] Bsec OK");
        }
        // BsecESuSampleratelimits
        BsecLibraryReturn::BsecESuSampleratelimits => {
            println!("[Rust][Callback] Invalid sample rate");
        }
        _ => {
            println!("[Rust][Callback] BSEC has error");
        }
    }

    // create a instance from struct and assign received values
    let bme680_data = Bme680SensorData{
        iaq, iaq_accuracy, temperature, humidity, pressure,
        raw_temperature: _raw_temperature,
        raw_humidity: _raw_humidity,
        gas, bsec_status,
        static_iaq: _static_iaq, 
        co2_equivalent, breath_voc_equivalent
    };

    // call bme680 channel data feeder method to feed obtain data from C 
    let _ = tokio::spawn(bme680_channel_feeder(bme680_data));
}

// method to forward data received from C callback to the allocated channel
async fn bme680_channel_feeder(bme680_data: Bme680SensorData){
    if let Some(data_tx) = CHANNEL_SENDER.get(){
        let res = data_tx.send(bme680_data).await;

        match res {
            Ok(_) => {},
            Err(err) => {
                eprintln!("Error Passing Message to Channel from BME Sensor: {:?}", err)
            }
        }
    }
}