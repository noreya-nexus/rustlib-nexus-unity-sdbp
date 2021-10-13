use std::io::{Error, ErrorKind};

use crate::sdbp::request::custom::bmc::protocol::*;
use crate::sdbp::response::custom::bmc::check_status;
use crate::sdbp::response::SdbpResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResetSuccess {
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetHubSuccess {
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetPortSuccess {
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct UsbHub {
    pub state: u8,
}

#[derive(Debug, Clone)]
pub struct UsbHubPort {
    pub port1: u8,
    pub port2: u8,
    pub port3: u8,
    pub port4: u8,
    pub port5: u8,
    pub port6: u8,
    pub port7: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsbHubRest {
    pub hub_state: String,
    pub port_slot_2: String,
    pub port_slot_3: String,
    pub port_slot_4: String,
    pub port_slot_5: String,
    pub port_slot_6: String,
    pub port_slot_7: String,
    pub port_slot_8: String,
}

pub fn map_state(state: u8) -> String {
    if state == 1 {
        return "enabled".to_string();
    }
    else if state == 2 {
        return "disabled".to_string();
    }
    else if state == 3 {
        return "not available".to_string();
    } else {
        warn!("Could not map usb state");
        return "not available".to_string();
    }
}

impl SdbpResponse for UsbHubPort {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 10 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::usbhub::ID ||
            value[2] != classes::usbhub::operation_code::GET_PORT_STATE {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let port1 = &value[3];
        let port2 = &value[4];
        let port3 = &value[5];
        let port4 = &value[6];
        let port5 = &value[7];
        let port6 = &value[8];
        let port7 = &value[9];
        let check: [u8; 7] = [*port1, *port2, *port3, *port4, *port5, *port6, *port7];

        for element in &check {
            if *element == 0 || *element > 3 {
                return Err(Error::new(ErrorKind::InvalidData, "invalid port state"));
            }
        }

        Ok(UsbHubPort {
            port1: port1.clone(),
            port2: port2.clone(),
            port3: port3.clone(),
            port4: port4.clone(),
            port5: port5.clone(),
            port6: port6.clone(),
            port7: port7.clone(),
        })
    }
}

impl SdbpResponse for UsbHub {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::usbhub::ID ||
            value[2] != classes::usbhub::operation_code::GET_HUB_STATE {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let hub_state = &value[3];

        if *hub_state == 0 || *hub_state > 3 {
            return Err(Error::new(ErrorKind::InvalidData, "invalid hub state"));
        }

        Ok(UsbHub {
            state: hub_state.clone(),
        })
    }
}

impl SdbpResponse for SetHubSuccess {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::usbhub::ID ||
            value[2] != classes::usbhub::operation_code::SET_HUB_STATE {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status, "setting hub state: ".to_string(), "State is invalid".to_string(), true) {
            Ok(msg) => Ok(SetHubSuccess { status: msg }),
            Err(err) => Err(err),
        }
    }
}

impl SdbpResponse for SetPortSuccess {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::usbhub::ID ||
            value[2] != classes::usbhub::operation_code::SET_PORT_STATE {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status, "setting port state: ".to_string(), "State is invalid".to_string(), true) {
            Ok(msg) => Ok(SetPortSuccess { status: msg }),
            Err(err) => Err(err),
        }
    }
}

impl SdbpResponse for ResetSuccess {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::usbhub::ID ||
            value[2] != classes::usbhub::operation_code::HUB_RESET {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status, "setting port state: ".to_string(), "State is invalid".to_string(), true) {
            Ok(msg) => Ok(ResetSuccess { status: msg }),
            Err(err) => Err(err),
        }
    }
}