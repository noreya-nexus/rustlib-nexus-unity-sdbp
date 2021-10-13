use std::io::{Error, ErrorKind};
use crate::sdbp::response::SdbpResponse;
use crate::sdbp::request::custom::io::protocol::*;

pub struct OutputModeStatus {
    pub status: u8,
    pub msg: String,
}


impl SdbpResponse for OutputModeStatus {

    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        trace!("{:?}",raw);

        let value = raw.as_slice();
        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::output_class::ID ||
            value[2] != classes::output_class::operation_code::SET_OUTPUT {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        let msg;
        match value[3] {

            0 => msg = "success".to_string(),
            1 => msg = "Invalid command".to_string(),
            2 => msg = "Wrong command length".to_string(),
            3 => msg = "Invalid mode".to_string(),
            4 => msg = "Invalid pin".to_string(),
            5 => msg = "Invalid value".to_string(),
            6 => msg = "Power config missing".to_string(),
            7 => msg = "External voltage".to_string(),
            _ => msg = "Unknown error code".to_string(),
        }

        Ok( OutputModeStatus{
            status: value[3], msg
        })
    }
}