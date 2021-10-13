use std::io::{Error, ErrorKind};
use crate::sdbp::response::SdbpResponse;
use crate::sdbp::request::custom::io::protocol::*;

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct TestPowerConfig {
  pub status : u8
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct SetPowerConfig {
    pub status : u8
}

impl SdbpResponse for TestPowerConfig {
    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
/*        if !(value.len() == 10 || value.len() == 4) {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }*/

        if value[0] != CLASS_ID ||
            value[1] != classes::power_management_class::ID ||
            value[2] != classes::power_management_class::operation_code::TEST_POWER_CONFIG {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        let mut status = (0,"success");
        if value.len() == 4 {
            status = match value[3] {
                0x00 => (0, "success"),
                0x02 => (2, "error"), //INVALID_MODE
                0x03 => (3, "error"), //FORCE_FAILED
                0x04 => (4, "error"), //FORCE_FAILED
                0x05 => (5, "error"), //FORCE_FAILED
                _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid value of status"))
            };
        }

        Ok(TestPowerConfig{
            status: status.0
        })
    }
}


impl SdbpResponse for SetPowerConfig {
    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        // if value.len() != 4 {
        //     return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        // }

        if value[0] != CLASS_ID ||
            value[1] != classes::power_management_class::ID ||
            value[2] != classes::power_management_class::operation_code::SET_POWER_CONFIG {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        let status = match value[3] {
            0x00 => (0, "success"),
            0x02 => (2, "error"), //INVALID_MODE
            0x03 => (3, "error"), //FORCE_FAILED
            0x04 => (4, "error"), //FORCE_FAILED
            0x05 => (5, "error"), //FORCE_FAILED
            _ => return Err(Error::new(ErrorKind::InvalidData,"Invalid value of status"))
        };


        Ok(SetPowerConfig{
            status: status.0,
        })
    }
}
