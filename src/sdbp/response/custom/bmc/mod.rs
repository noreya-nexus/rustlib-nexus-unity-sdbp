pub mod voltage;
pub mod buzzer;
pub mod watchdog;
pub mod cmc;
pub mod usbhub;

use std::io::{ErrorKind,Error};
use crate::sdbp::request::custom::bmc::protocol::classes;

pub fn check_status(status: &u8, prefix: String, error_msg: String, show_error: bool) -> Result<String, Error> {
    for error in &classes::return_code::ERR_LIST {
        if error.0 == *status {
            return if show_error {
                Err(Error::new(ErrorKind::InvalidData, format!("{}{}", prefix, error.1)))
            } else {
                Err(Error::new(ErrorKind::InvalidData, prefix))
            }
        }
    }

    if *status == classes::return_code::OK.0 {
        Ok(classes::return_code::OK.1.to_string())
    } else {
        Err(Error::new(ErrorKind::InvalidData, error_msg))
    }
}