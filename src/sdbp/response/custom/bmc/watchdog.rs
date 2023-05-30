use std::io::{Error, ErrorKind};

use crate::sdbp::request::custom::bmc::protocol::*;
use crate::sdbp::response::SdbpResponse;
use crate::sdbp::response::custom::bmc::check_status;

pub mod json_response {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Timeout {
        pub timeout: u32, // IMPROVEMENT: Add unit suffix
        pub timeout_left: u32, // IMPROVEMENT: Add unit suffix
        pub shutdown_timeout: u32, // IMPROVEMENT: Add unit suffix
        pub emergency_mode: bool,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct SwShutdown {
        pub status: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct TimeoutResponse {
        pub status: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct AliveResponse {
        pub status: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct SaveConfig {
        pub status: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct SetShutdownTimeout {
        pub status: String,
    }
}

pub mod ipc {
    #[derive(Debug, Clone)]
    pub struct GetTimeout {
        pub timeout: u32, // IMPROVEMENT: Add unit suffix
    }

    #[derive(Debug, Clone)]
    pub struct GetEmergency {
        pub status: bool,
    }
}

impl SdbpResponse for json_response::TimeoutResponse {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::watchdog::ID ||
            !(value[2] == classes::watchdog::operation_code::ENABLE_TIMEOUT
                || value[2] == classes::watchdog::operation_code::DISABLE_TIMEOUT) {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status,"timeout: ".to_string(),"State is invalid".to_string(),true)  {
            Ok(msg) => Ok(json_response::TimeoutResponse { status: msg, }),
            Err(err) => Err(err),
        }
    }
}


impl SdbpResponse for ipc::GetTimeout {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 + 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::watchdog::ID ||
            !(value[2] == classes::watchdog::operation_code::GET_TIMEOUT
                || value[2] == classes::watchdog::operation_code::GET_TIME_LEFT
                || value[2] == classes::watchdog::operation_code::GET_SHUTDOWN_TIMEOUT) {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status,"timeout: ".to_string(),"State is invalid".to_string(),true)  {
            Ok(msg) => msg,
            Err(err) => return Err(err),
        };

        let timeout = u32::from_be_bytes([value[4], value[5], value[6], value[7]]);

        Ok(ipc::GetTimeout {
            timeout,
        })
    }
}

impl SdbpResponse for ipc::GetEmergency {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::watchdog::ID ||
            !(value[2] == classes::watchdog::operation_code::EMERGENCY_MODE_STATE) {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        if status.clone() == 0 {  // DISABLED
            Ok(ipc::GetEmergency { status: false, })
        } else if status.clone() == 1 {
            Ok(ipc::GetEmergency { status: true, })
        }
        else {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid emergency mode state"));
        }
    }
}

impl SdbpResponse for json_response::AliveResponse {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::watchdog::ID ||
            !(value[2] == classes::watchdog::operation_code::ALIVE) {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status,"watchdog timeout not enabled".to_string(),"State is invalid".to_string(), false)  {
            Ok(msg) => Ok(json_response::AliveResponse { status:  msg, }),
            Err(err) => Err(err),
        }
    }
}

impl SdbpResponse for json_response::SaveConfig {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::watchdog::ID ||
            !(value[2] == classes::watchdog::operation_code::SAVE_CONFIG) {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status,"save config failed: ".to_string(),"State is invalid".to_string(), true)  {
            Ok(msg) => Ok(json_response::SaveConfig { status:  msg, }),
            Err(err) => Err(err),
        }
    }
}

impl SdbpResponse for json_response::SwShutdown {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::watchdog::ID ||
            !(value[2] == classes::watchdog::operation_code::SW_SHUTDOWN) {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status,"shutdown failed: ".to_string(),"State is invalid".to_string(),true)  {
            Ok(msg) => Ok(json_response::SwShutdown { status:  msg, }),
            Err(err) => Err(err),
        }
    }
}

impl SdbpResponse for json_response::SetShutdownTimeout {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::watchdog::ID ||
            !(value[2] == classes::watchdog::operation_code::SET_SHUTDOWN_TIMEOUT) {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status,"setting shutdown timeout: ".to_string(),"State is invalid".to_string(),true)  {
            Ok(msg) => Ok(json_response::SetShutdownTimeout { status:  msg, }),
            Err(err) => Err(err),
        }
    }
}
