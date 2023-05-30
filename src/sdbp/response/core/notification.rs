use crate::sdbp::request::core::protocol::classes::notification::*;
use crate::sdbp::request::core::*;
use crate::sdbp::response::SdbpResponse;
use std::io::{Error, ErrorKind};

pub struct NotificationResponse {
    pub notification: Vec<u8>,
}

impl SdbpResponse for NotificationResponse {
    fn from_raw(raw: Vec<u8>) -> Result<Self, Error> {
        trace!("{:?}", raw);

        let value = raw.as_slice();
        if value.len() < 4 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid length {}", value.len()),
            ));
        }

        if value[0] != protocol::CLASS_ID || value[1] != ID {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        if value[2] == operation_code::GET_NOTIFICATION {
            let mut notification = vec![0;4];
            notification.copy_from_slice(&value[4..]);
            Ok(NotificationResponse { notification })
        } else if value[2] == operation_code::ERROR
            && value[3] == return_code::NO_NOTIFICATION_PENDING
        {
            // No notification pending
            return Ok(NotificationResponse {
                notification: vec![0, 0, 0, 0],
            });
        } else {
            let msg: String;
            match value[3] {
                return_code::COMMAND_INVALID => msg = "Invalid command".to_string(),
                return_code::WRONG_LENGTH => msg = "Wrong command length".to_string(),
                _ => msg = "Unknown notification error".to_string(),
            }
            return Err(Error::new(ErrorKind::InvalidData, msg));
        }
    }
}
