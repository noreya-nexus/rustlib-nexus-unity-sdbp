use std::io::{ErrorKind,Error};
use crate::sdbp::response::SdbpResponse;
use crate::sdbp::request::custom::bmc::protocol::*;
use crate::sdbp::response::custom::bmc::check_status;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Buzzer {
    pub status : String,
}

impl SdbpResponse for Buzzer {

    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {

        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData,"Invalid length"));
        }
        
        if  value[0] != CLASS_ID ||
            value[1] != classes::buzzer::ID ||
            value[2] != classes::buzzer::operation_code::MODE_BUZZER {
            return Err(Error::new(ErrorKind::InvalidData,"Wrong Header"))
        }

        let status = &value[3];

        match check_status(status,"buzzer: ".to_string(),"State is invalid".to_string(),true)  {
            Ok(_) => Ok(Buzzer { status: "success".to_string(), }),
            Err(err) => Err(err),
        }
    }
}