use std::io::{Error, ErrorKind};
use crate::sdbp::response::SdbpResponse;
use crate::sdbp::request::custom::io::protocol::*;

pub struct InputModeStatus {
    pub status: u8,
    pub msg: String,
}

pub struct AnalogThresholdStatus {
    pub status: u8,
    pub msg: String,
}

pub struct DigitalInterruptStatus {
    pub status: u8,
    pub msg: String,
}

pub struct DigitalCounterStatus {
    pub status: u8,
    pub msg: String,
}

#[derive(Debug,serde::Serialize, serde::Deserialize)]
pub struct PinValues {

    pub pin: u8,
    pub pin_type: String,
    pub value: u32,

}


const VALUE_TYPE_VOLTAGE : u8 = 0x00;
const VALUE_TYPE_CURRENT : u8 = 0x01;
const VALUE_TYPE_DIGITAL_INPUT_STATE : u8 = 0x02;
const VALUE_TYPE_FREQUENCY_COUNTER : u8 = 0x03;

impl PinValues {

    pub fn new_empty() -> PinValues {
        return PinValues{pin : 0, pin_type: "Invalid".to_string(), value: 0}
    }

    pub fn set_id(&mut self, pin : u8) {
        self.pin  = pin
    }

    pub fn set_voltage(&mut self, input : &[u8]) {
        if input.len() == 2 {
            let mut voltage = 0;
            voltage |= (input[0] as u32) << 8;
            voltage |= (input[1] as u32) << 0;
            self.pin_type = "voltage_millivolt".to_string();
            self.value = voltage;
        }

    }

    pub fn set_current(&mut self, input : &[u8]) {
        if input.len() == 2 {
            let mut current = 0;
            current |= (input[0] as u32) << 8;
            current |= (input[1] as u32) << 0;
            self.pin_type = "current_milliampere".to_string();
            self.value = current;

        }
    }

    pub fn set_digital_state(&mut self, input : &[u8]) {
        if input.len() == 2 {
            let mut digital_input_state = 0;
            digital_input_state |= (input[0] as u32) << 8;
            digital_input_state |= input[1] as u32;
            self.pin_type = "digital_input".to_string();
            self.value = digital_input_state;
        }

    }

    pub fn set_frequency_counter(&mut self, input : &[u8]) {
        if input.len() == 4 {
            let mut frequency_counter_value = 0;
            frequency_counter_value |= (input[0] as u32) << 24;
            frequency_counter_value |= (input[1] as u32) << 16;
            frequency_counter_value |= (input[2] as u32) << 8;
            frequency_counter_value |= (input[3] as u32) << 0;
            self.pin_type = "frequency_hertz".to_string();
            self.value = frequency_counter_value;
        }
    }
}

#[derive(Debug,serde::Serialize, serde::Deserialize)]
pub struct GetValuesStatus {
    pub pins : Vec<PinValues>,
}


impl GetValuesStatus {

    pub fn new_empty(self) -> GetValuesStatus{

        let mut pins = vec![];
        for _ in 0..6 {
            pins.push(PinValues::new_empty())

        }
        return GetValuesStatus {pins}
    }
}


impl SdbpResponse for GetValuesStatus {
    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        if value.len() < 4 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::input_class::ID ||
            (value[2] != classes::input_class::operation_code::GET_VALUES && value[2] != classes::input_class::operation_code::GET_CURRENT_VALUES)
        {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        let mut idx = 3;

        let msg;
        match value[3] {
            0 => msg = "success".to_string(),
            1 => msg = "Invalid command".to_string(),
            2 => msg = "Wrong command length".to_string(),
            3 => msg = "Invalid rail".to_string(),
            4 => msg = "Invalid pin".to_string(),
            5 => msg = "Invalid mode".to_string(),
            _ => msg = "Unknown error code".to_string(),
        }

        if msg != "success"{
            return Err(Error::new(ErrorKind::InvalidData, msg.to_string()));
        }

        let mut result = GetValuesStatus {pins: vec![]};

        idx += 1;
        let mut bytes_left = raw.len() - idx;

        trace!("{:?}",raw);

        let mut pins = vec![];

        while bytes_left > 3 {

            let pin_id = value[idx];
            idx +=1;
            let value_type = value[idx];
            idx += 1;
            let value_length = value[idx];
            idx += 1;
            let value = &value[idx as usize ..(idx as usize + value_length as usize)];
            idx += value_length as usize;

            let mut pin = PinValues::new_empty();


            pin.set_id(pin_id);
            if value_type == VALUE_TYPE_VOLTAGE {
                pin.set_voltage(value);
            }
            if value_type == VALUE_TYPE_CURRENT {
                pin.set_current(value);
            }

            if value_type == VALUE_TYPE_FREQUENCY_COUNTER {
                pin.set_frequency_counter(value);
            }

            if value_type == VALUE_TYPE_DIGITAL_INPUT_STATE {
                pin.set_digital_state(value);
            }
            pins.push(pin);

            bytes_left = raw.len() - idx;
        }

        result.pins = pins;
        return Ok(result)
    }
}


impl SdbpResponse for InputModeStatus {

    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::input_class::ID ||
            value[2] != classes::input_class::operation_code::SET_INPUT_MODE {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        let msg;
        match value[3] {

            0 => msg = "success".to_string(),
            1 => msg = "Invalid command".to_string(),
            2 => msg = "Wrong command length".to_string(),
            3 => msg = "Invalid rail".to_string(),
            4 => msg = "Invalid pin".to_string(),
            5 => msg = "Invalid mode".to_string(),
            _ => msg = "Unknown error code".to_string(),
        }


        Ok( InputModeStatus{
            status: value[3], msg
        })
    }
}

impl SdbpResponse for AnalogThresholdStatus {

    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::input_class::ID ||
            value[2] != classes::input_class::operation_code::SET_ANALOG_THRESHOLD {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        let msg;
        match value[3] {

            0 => msg = "success".to_string(),
            1 => msg = "Invalid command".to_string(),
            2 => msg = "Wrong command length".to_string(),
            3 => msg = "Invalid mode".to_string(),
            4 => msg = "Invalid pin".to_string(),
            5 => msg = "Invalid direction".to_string(),
            6 => msg = "Invalid value".to_string(),
            _ => msg = "Unknown error code".to_string(),
        }

        Ok( AnalogThresholdStatus{
            status: value[3], msg
        })
    }
}


impl SdbpResponse for DigitalInterruptStatus {

    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::input_class::ID ||
            value[2] != classes::input_class::operation_code::SET_DIGITAL_INTERRUPT {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        let msg;
        match value[3] {

            0 => msg = "success".to_string(),
            1 => msg = "Invalid command".to_string(),
            2 => msg = "Wrong command length".to_string(),
            3 => msg = "Invalid mode".to_string(),
            4 => msg = "Invalid pin".to_string(),
            5 => msg = "Invalid direction".to_string(),
            6 => msg = "Invalid value".to_string(),
            _ => msg = "Unknown error code".to_string(),
        }

        Ok( DigitalInterruptStatus{
            status: value[3], msg
        })
    }
}

impl SdbpResponse for DigitalCounterStatus {

    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::input_class::ID ||
            value[2] != classes::input_class::operation_code::SET_DIGITAL_COUNTER {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        let msg;
        match value[3] {

            0 => msg = "success".to_string(),
            1 => msg = "Invalid command".to_string(),
            2 => msg = "Wrong command length".to_string(),
            3 => msg = "Invalid mode".to_string(),
            4 => msg = "Invalid pin".to_string(),
            5 => msg = "Invalid direction".to_string(),
            6 => msg = "Invalid value".to_string(),
            _ => msg = "Unknown error code".to_string(),
        }

        Ok( DigitalCounterStatus{
            status: value[3], msg
        })
    }
}