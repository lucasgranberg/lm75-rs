use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use lm75::{Address, FaultQueue, Lm75, OsMode, OsPolarity};

mod common;

use crate::common::{assert_invalid_input_data_error, Register, ADDR};

#[test]
fn can_enable() {
    let transaction = &[I2cTrans::write(ADDR, vec![Register::CONFIGURATION, 0])];
    let mut i2c = I2cMock::new(transaction);
    let mut sensor = Lm75::new(&mut i2c, Address::default());
    sensor.enable().unwrap();
}

#[test]
fn can_disable() {
    let transaction = &[I2cTrans::write(ADDR, vec![Register::CONFIGURATION, 1])];
    let mut i2c = I2cMock::new(transaction);
    let mut sensor = Lm75::new(&mut i2c, Address::default());
    sensor.disable().unwrap();
}

#[test]
fn can_read_temperature() {
    let transaction = &[I2cTrans::write_read(
        ADDR,
        vec![Register::TEMPERATURE],
        vec![0b1110_0111, 0b1010_0101], // -24.5
    )];
    let mut i2c = I2cMock::new(transaction);
    let mut sensor = Lm75::new(&mut i2c, Address::default());
    let temp = sensor.read_temperature().unwrap();
    assert!(-24.4 > temp);
    assert!(-24.6 < temp);
}

#[test]
fn can_read_temperature_pct2075() {
    let transaction = &[I2cTrans::write_read(
        ADDR,
        vec![Register::TEMPERATURE],
        vec![0b1110_0111, 0b1010_0101], // -24.375
    )];
    let mut i2c = I2cMock::new(transaction);
    let mut sensor = Lm75::new(&mut i2c, Address::default());
    let temp = sensor.read_temperature().unwrap();
    assert!(-24.3 > temp);
    assert!(-24.4 < temp);
}

#[test]
fn can_read_sample_rate() {
    let transaction = &[I2cTrans::write_read(
        ADDR,
        vec![Register::T_IDLE],
        vec![0b0000_0001], // 100ms
    )];
    let mut i2c = I2cMock::new(transaction);
    let mut sensor = Lm75::new_pct2075(&mut i2c, Address::default());
    let period = sensor.read_sample_rate().unwrap();
    assert_eq!(100, period);
}

macro_rules! set_config_test {
    ( $test_name:ident, $method:ident, $value:expr, $expected:expr ) => {
        #[test]
        fn $test_name() {
            let transaction = &[I2cTrans::write(
                ADDR,
                vec![Register::CONFIGURATION, $expected],
            )];
            let mut i2c = I2cMock::new(transaction);
            let mut sensor = Lm75::new(&mut i2c, Address::default());
            sensor.$method($value).unwrap();
        }
    };
}

set_config_test!(
    can_set_fault_queue_1,
    set_fault_queue,
    FaultQueue::_1,
    0b0000_0000
);
set_config_test!(
    can_set_fault_queue_2,
    set_fault_queue,
    FaultQueue::_2,
    0b0000_1000
);
set_config_test!(
    can_set_fault_queue_4,
    set_fault_queue,
    FaultQueue::_4,
    0b0001_0000
);
set_config_test!(
    can_set_fault_queue_6,
    set_fault_queue,
    FaultQueue::_6,
    0b0001_1000
);

set_config_test!(
    can_set_os_polarity_low,
    set_os_polarity,
    OsPolarity::ActiveLow,
    0b0000_0000
);
set_config_test!(
    can_set_os_polarity_high,
    set_os_polarity,
    OsPolarity::ActiveHigh,
    0b0000_0100
);

set_config_test!(
    can_set_os_mode_low,
    set_os_mode,
    OsMode::Comparator,
    0b0000_0000
);
set_config_test!(
    can_set_os_mode_high,
    set_os_mode,
    OsMode::Interrupt,
    0b0000_0010
);

macro_rules! set_temp_test {
    ( $test_name:ident, $method:ident, $value:expr, $register:expr,
      $expected_msb:expr, $expected_lsb:expr ) => {
        #[test]
        fn $test_name() {
            let transaction = &[I2cTrans::write(
                ADDR,
                vec![$register, $expected_msb, $expected_lsb],
            )];
            let mut i2c = I2cMock::new(transaction);
            let mut sensor = Lm75::new(&mut i2c, Address::default());
            sensor.$method($value).unwrap();
        }
    };
}

set_temp_test!(
    can_set_os_temp_0_5,
    set_os_temperature,
    0.5,
    Register::T_OS,
    0b0000_0000,
    0b1000_0000
);
set_temp_test!(
    can_set_os_temp_min,
    set_os_temperature,
    -55.0,
    Register::T_OS,
    0b1100_1001,
    0
);
set_temp_test!(
    can_set_os_temp_max,
    set_os_temperature,
    125.0,
    Register::T_OS,
    0b0111_1101,
    0
);

macro_rules! invalid_temp_test {
    ($test_name:ident, $method:ident, $value:expr) => {
        #[test]
        fn $test_name() {
            let mut i2c = I2cMock::new(&[]);
            let mut sensor = Lm75::new_pct2075(&mut i2c, Address::default());
            assert_invalid_input_data_error(sensor.$method($value));
        }
    };
}

invalid_temp_test!(set_os_temperature_too_low, set_os_temperature, -55.5);
invalid_temp_test!(set_os_temperature_too_high, set_os_temperature, 125.5);

set_temp_test!(
    can_set_hyst_temp_0_5,
    set_hysteresis_temperature,
    0.5,
    Register::T_HYST,
    0b0000_0000,
    0b1000_0000
);
set_temp_test!(
    can_set_hyst_temp_min,
    set_hysteresis_temperature,
    -55.0,
    Register::T_HYST,
    0b1100_1001,
    0
);
set_temp_test!(
    can_set_hyst_temp_max,
    set_hysteresis_temperature,
    125.0,
    Register::T_HYST,
    0b0111_1101,
    0
);

invalid_temp_test!(
    set_hyst_temperature_too_low,
    set_hysteresis_temperature,
    -55.5
);
invalid_temp_test!(
    set_hyst_temperature_too_high,
    set_hysteresis_temperature,
    125.5
);

macro_rules! set_sample_rate_test {
    ( $test_name:ident, $method:ident, $value:expr, $register:expr,
      $period:expr) => {
        #[test]
        fn $test_name() {
            let transaction = &[I2cTrans::write(ADDR, vec![$register, $period])];
            let mut i2c = I2cMock::new(transaction);
            let mut sensor = Lm75::new_pct2075(&mut i2c, Address::default());
            sensor.$method($value).unwrap();
        }
    };
}

set_sample_rate_test!(
    can_set_max_sample_rate,
    set_sample_rate,
    3100,
    Register::T_IDLE,
    0b0001_1111
);
set_sample_rate_test!(
    can_set_custom_sample_rate,
    set_sample_rate,
    1500,
    Register::T_IDLE,
    0b0000_1111
);
set_sample_rate_test!(
    can_set_default_sample_rate,
    set_sample_rate,
    100,
    Register::T_IDLE,
    0b0000_0001
);

macro_rules! invalid_sample_rate_test {
    ($test_name:ident, $method:ident, $value:expr) => {
        #[test]
        fn $test_name() {
            let mut i2c = I2cMock::new(&[]);
            let mut sensor = Lm75::new_pct2075(&mut i2c, Address::default());
            assert_invalid_input_data_error(sensor.$method($value));
        }
    };
}

invalid_sample_rate_test!(set_sample_rate_too_high, set_sample_rate, 4000);
invalid_sample_rate_test!(set_non_multiple_sample_rate, set_sample_rate, 1234);
