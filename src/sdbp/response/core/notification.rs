use std::io::{Error, ErrorKind};
use crate::sdbp::response::SdbpResponse;
use crate::sdbp::request::core::*;

pub struct NotificationResponse {
    pub notification: Vec<u8>
}


impl SdbpResponse for NotificationResponse {

    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {

        trace!("{:?}",raw);

        let value = raw.as_slice();
        if value.len() < 4 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != protocol::CLASS_ID ||
            value[1] != protocol::classes::notification::ID {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"))
        }

        if value.len() == 4 { // If error response
            let msg;
            if value[3] == 3 { // No notification pending
                return Ok( NotificationResponse{
                    notification: vec![0,0,0,0]
                })
            }
            match value[3] {
                0 => msg = "Ok".to_string(),
                1 => msg = "Invalid command".to_string(),
                2 => msg = "Wrong command length".to_string(),
                3 => msg = "No notification pending".to_string(),
                _ => msg = "Unknown error code".to_string(),
            }
            return  Err(Error::new(ErrorKind::InvalidData, msg))
        }
        else {
            let mut notification = Vec::new();
            //Array copy not implemented in Rust currently
            for entry in &value[3..] {
                notification.push(*entry);
            }
            Ok( NotificationResponse{
                notification
            })
        }
    }
}