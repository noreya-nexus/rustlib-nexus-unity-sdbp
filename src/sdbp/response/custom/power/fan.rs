use std::io::{Error, ErrorKind};
use crate::sdbp::response::SdbpResponse;

use crate::sdbp::request::custom::power::protocol::*;

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct FanStatus {
    pub fan_forced: bool,
    pub fan_current_mode : u8,
    pub fan_setting_mode : u8
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct RpmStatus {
    pub enabled: bool,
    pub rpm : u16,
}


#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct RpmControl {
    pub status: String,
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct FanControl {
    pub status: String,
}

impl SdbpResponse for FanControl {
    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::temperature_control_class::ID ||
            value[2] != classes::temperature_control_class::operation_code::FAN_CONTROL {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        let status = match value[3] {
            1 => (1, "success"),
            2 => (2, "error"), //INVALID_MODE
            3 => (3, "error"), //FORCE_FAILED
            _ => return Err(Error::new(ErrorKind::InvalidData,"Invalid value of status"))
        };
        
        Ok(FanControl{
            status: status.1.to_string()
        })
    }
}


impl SdbpResponse for RpmControl {
    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::temperature_control_class::ID ||
            value[2] != classes::temperature_control_class::operation_code::FAN_RPM_CONTROL{
            return Err(Error::new(ErrorKind::InvalidData,"Wrong Header"))
        }
        
        let enabled = match value[3] {
            1 => "success",
            2 => "error",
            _ => return Err(Error::new(ErrorKind::InvalidData,"Invalid value of measurement enabled"))
        };
        
        Ok(RpmControl{
            status: enabled.to_string()
        })
    }
}


impl SdbpResponse for RpmStatus {
    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        if value.len() != 6 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::temperature_control_class::ID ||
            value[2] != classes::temperature_control_class::operation_code::FAN_RPM {
            return Err(Error::new(ErrorKind::InvalidData,"Wrong Header"))
        }


        let enabled = match value[3] {
            1 => true,
            2 => false,
            _ => return Err(Error::new(ErrorKind::InvalidData,"Invalid value of measurement enabled"))
        };

        let rpm = u16::from_be_bytes([value[4],value[5]]);

        Ok(RpmStatus{
            enabled: enabled,
            rpm: rpm,
        })

    }
}

impl SdbpResponse for FanStatus {
    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        if value.len() != 6 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::temperature_control_class::ID ||
            value[2] != classes::temperature_control_class::operation_code::FAN_STATUS {
            return Err(Error::new(ErrorKind::InvalidData,"Wrong Header"))
        }

        let forced = match value[3] {
            1 => true,
            2 => false,
            _ => return Err(Error::new(ErrorKind::InvalidData,"Invalid value for fan forced")),
        };

        let current_mode : u8 = match value[4] {
            0 => 0,
            1 => 100,
            3 => 20,
            4 => 40,
            5 => 60,
            6 => 80,
            _ => return Err(Error::new(ErrorKind::InvalidData,"Invalid value for fan current mode")),
        };

        let setting_mode : u8 = match value[5] {
            0 => 0,
            1 => 100,
            3 => 20,
            4 => 40,
            5 => 60,
            6 => 80,
            _ => return Err(Error::new(ErrorKind::InvalidData,"Invalid value for fan setting mode")),
        };

        Ok(FanStatus{
            fan_forced: forced,
            fan_current_mode: current_mode,
            fan_setting_mode: setting_mode
        })
    }
}