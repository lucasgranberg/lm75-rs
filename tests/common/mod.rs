use lm75::Error;

pub const ADDR: u8 = 0b100_1000;

pub struct Register;

impl Register {
    pub const TEMPERATURE: u8 = 0x00;
    pub const CONFIGURATION: u8 = 0x01;
    pub const T_HYST: u8 = 0x02;
    pub const T_OS: u8 = 0x03;
    pub const T_IDLE: u8 = 0x04;
}

pub fn assert_invalid_input_data_error<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::InvalidInputData) => (),
        _ => panic!("Did not return Error::InvalidInputData."),
    }
}
