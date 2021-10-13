use std::io::{Error, ErrorKind};
use crate::sdbp::response::SdbpResponse;
use crate::sdbp::request::core::*;

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct SuspendResponse {
    pub status: String,
}

impl SdbpResponse for SuspendResponse {

    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        if value.len() < 4 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != protocol::CLASS_ID ||
            value[1] != protocol::classes::control::ID {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        if value[2] != protocol::classes::control::operation_code::MODE_SUSPEND || value[3] != 0x00 {
            return Err(Error::new(ErrorKind::InvalidData, "Response is invalid"))
        }

        Ok( SuspendResponse{
            status:  "success".to_string()
        })
    }
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct RunResponse {
    pub status: String,
}

impl SdbpResponse for RunResponse {

    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        let value = raw.as_slice();
        if value.len() < 4 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != protocol::CLASS_ID ||
            value[1] != protocol::classes::control::ID {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        if value[2] != protocol::classes::control::operation_code::MODE_RUN || value[3] != 0x00 {
            return Err(Error::new(ErrorKind::InvalidData, "Response is invalid"))
        }

        Ok( RunResponse{
            status:  "success".to_string()
        })
    }
}