use std::io::{Error, ErrorKind};

use crate::sdbp::request::custom::bmc::protocol::*;
use crate::sdbp::response::custom::bmc::check_status;
use crate::sdbp::response::SdbpResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cmc {
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Reset {
    pub status: String,
}

impl SdbpResponse for Cmc {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::cmc::ID ||
            value[2] != classes::cmc::operation_code::CTL_USBBOOT {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status, "enabling usb bootloader: ".to_string(), "State is invalid".to_string(), true) {
            Ok(msg) => Ok(Cmc { status: msg }),
            Err(err) => Err(err),
        }
    }
}

impl SdbpResponse for Reset {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != classes::control::CLASS_ID ||
            value[1] != classes::control::ID ||
            value[2] != classes::control::operation_code::SYSTEM_RESET {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status, "hard reset: ".to_string(), "State is invalid".to_string(), true) {
            Ok(msg) => Ok(Reset { status: msg }),
            Err(err) => Err(err),
        }
    }
}