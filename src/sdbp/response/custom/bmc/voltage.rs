use std::io::{ErrorKind,Error};
use crate::sdbp::response::SdbpResponse;
use crate::sdbp::request::custom::bmc::protocol::*;
use crate::sdbp::response::custom::bmc::check_status;

#[derive(Debug,Clone)]
pub struct Voltage {
    pub voltage : u32,
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct VoltageRest {
    pub voltage_1v8 : u32, // IMPROVEMENT: Add unit suffix
    pub voltage_rtc : u32, // IMPROVEMENT: Add unit suffix
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct TemperatureRest {
    #[serde(serialize_with = "round_temp")]
    pub ntc_0 : f32, // IMPROVEMENT: Add unit suffix
    #[serde(serialize_with = "round_temp")]
    pub ntc_1 : f32, // IMPROVEMENT: Add unit suffix
}

fn round_temp<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    let value = (x * 1000.0).round() / 1000.0;
    s.serialize_f32(value)
}

impl SdbpResponse for Voltage {

    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {

        let value = value.as_slice();

        if value.len() != 8 {
            return Err(Error::new(ErrorKind::InvalidData,"Invalid length"));
        }

        if  value[0] != CLASS_ID ||
            value[1] != classes::input::ID ||
            value[2] != classes::input::operation_code::GET_VOLTAGE {
            return Err(Error::new(ErrorKind::InvalidData,"Wrong Header"))
        }

        let status = &value[3];
        let voltage = &value[4..8];
        let voltage = u32::from_ne_bytes([voltage[3], voltage[2], voltage[1], voltage[0]]);

        match check_status(status,"setting shutdown timeout: ".to_string(),"State is invalid".to_string(),true)  {
            Ok(_) => Ok(Voltage { voltage, }),
            Err(err) => Err(err),
        }
    }
}