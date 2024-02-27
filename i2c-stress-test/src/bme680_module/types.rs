// struct used to pass BME680 sensor data
pub struct Bme680SensorData{
    pub iaq: f32,
    pub iaq_accuracy: u8,
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub raw_temperature: f32,
    pub raw_humidity: f32,
    pub gas: f32,
    pub bsec_status: BsecLibraryReturn,
    pub static_iaq: f32,
    pub co2_equivalent: f32,
    pub breath_voc_equivalent: f32
}

pub enum BsecLibraryReturn {
    BsecOk = 0,                                 // < Function execution successful
    BsecEDostepsInvalidinput = -1, // < Input (physical) sensor id passed to bsec_do_steps() is not in the valid range or not valid for requested virtual sensor
    BsecEDostepsValuelimits = -2, // < Value of input (physical) sensor signal passed to bsec_do_steps() is not in the valid range
    BsecEDostepsDuplicateinput = -6, // < Duplicate input (physical) sensor ids passed as input to bsec_do_steps()
    BsecIDostepsNooutputsreturnable = 2, // < No memory allocated to hold return values from bsec_do_steps(), i.e., n_outputs == 0
    BsecWDostepsExcessoutputs = 3, // < Not enough memory allocated to hold return values from bsec_do_steps(), i.e., n_outputs < maximum number of requested output (virtual) sensors
    BsecWDostepsTsintradiffoutofrange = 4, // < Duplicate timestamps passed to bsec_do_steps()
    BsecESuWrongdatarate = -10, // < The sample_rate of the requested output (virtual) sensor passed to bsec_update_subscription() is zero
    BsecESuSampleratelimits = -12, // < The sample_rate of the requested output (virtual) sensor passed to bsec_update_subscription() does not match with the sampling rate allowed for that sensor
    BsecESuDuplicategate = -13, // < Duplicate output (virtual) sensor ids requested through bsec_update_subscription()
    BsecESuInvalidsamplerate = -14, // < The sample_rate of the requested output (virtual) sensor passed to bsec_update_subscription() does not fall within the global minimum and maximum sampling rates
    BsecESuGatecountexceedsarray = -15, // < Not enough memory allocated to hold returned input (physical) sensor data from bsec_update_subscription(), i.e., n_required_sensor_settings < #BSEC_MAX_PHYSICAL_SENSOR
    BsecESuSamplintvlintegermult = -16, // < The sample_rate of the requested output (virtual) sensor passed to bsec_update_subscription() is not correct
    BsecESuMultgassamplintvl = -17, // < The sample_rate of the requested output (virtual), which requires the gas sensor, is not equal to the sample_rate that the gas sensor is being operated
    BsecESuHighheateronduration = -18, // < The duration of one measurement is longer than the requested sampling interval
    BsecWSuUnknownoutputgate = 10, // < Output (virtual) sensor id passed to bsec_update_subscription() is not in the valid range; e.g., n_requested_virtual_sensors > actual number of output (virtual) sensors requested
    BsecWSuModinnoulp = 11,        // < ULP plus can not be requested in non-ulp mode MOD_ONLY
    BsecISuSubscribedoutputgates = 12, // < No output (virtual) sensor data were requested via bsec_update_subscription()
    BsecEParseSectionexceedsworkbuffer = -32, // < n_work_buffer_size passed to bsec_set_[configuration/state]() not sufficient
    BsecEConfigFail = -33,                    // < Configuration failed
    BsecEConfigVersionmismatch = -34, // < Version encoded in serialized_[settings/state] passed to bsec_set_[configuration/state]() does not match with current version
    BsecEConfigFeaturemismatch = -35, // < Enabled features encoded in serialized_[settings/state] passed to bsec_set_[configuration/state]() does not match with current library implementation
    BsecEConfigCrcmismatch = -36, // < serialized_[settings/state] passed to bsec_set_[configuration/state]() is corrupted
    BsecEConfigEmpty = -37, // < n_serialized_[settings/state] passed to bsec_set_[configuration/state]() is to short to be valid
    BsecEConfigInsufficientworkbuffer = -38, // < Provided work_buffer is not large enough to hold the desired string
    BsecEConfigInvalidstringsize = -40, // < String size encoded in configuration/state strings passed to bsec_set_[configuration/state]() does not match with the actual string size n_serialized_[settings/state] passed to these functions
    BsecEConfigInsufficientbuffer = -41, // < String buffer insufficient to hold serialized data from BSEC library
    BsecESetInvalidchannelidentifier = -100, // < Internal error code, size of work buffer in setConfig must be set to BSEC_MAX_WORKBUFFER_SIZE
    BsecESetInvalidlength = -104,            // < Internal error code */
    BsecWScCallTimingViolation = 100, // < Difference between actual and defined sampling intervals of bsec_sensor_control() greater than allowed
    BsecWScModexceedulptimelimit = 101, // < ULP plus is not allowed because an ULP measurement just took or will take place
    BsecWScModinsufficientwaittime = 102, // < ULP plus is not allowed because not sufficient time passed since last ULP plus
}

// BME Sensor Sample Rate (low power mode)
pub const BSEC_SAMPLE_RATE_LP: f32                      =  0.33333;

// BME Sensor Temperature offset
pub const BSEC_TEMPERATURE_OFFSET: f32                  = 3.0;