use std::io::Error;
use std::io::ErrorKind;
use crate::sdbp::response::SdbpResponse;
use crate::sdbp::request::custom::power::protocol::*;


#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseTemperature {
    #[serde(serialize_with = "round_temp")]
    pub on_board_temp: f32, // IMPROVEMENT: Add unit suffix
    #[serde(serialize_with = "round_temp")]
    pub pmc_temp: f32 // IMPROVEMENT: Add unit suffix
}


fn round_temp<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    let value = (x * 1000.0).round() / 1000.0;
    s.serialize_f32(value)
}

impl SdbpResponse for ResponseTemperature {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {

        let value = value.as_slice();
        if value.len() != 7 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::temperature_control_class::ID ||
            value[2] != classes::temperature_control_class::operation_code::TEMPERATURE_SENSOR {
            return Err(Error::new(ErrorKind::InvalidData,"Wrong Header"))
        }

        let onboard_voltage  = u16::from_be_bytes([value[3],value[4]]);
        let pmc_temp : f32 = u16::from_be_bytes([value[5],value[6]]) as f32 / 9.5 as f32;

        let ntc0_volt : f32 = onboard_voltage as f32 / 1000.0;
        let res : f32  = (3.0 - ntc0_volt) / (ntc0_volt / 10000.0);

        let beta  = 3380.0;
        let ro = 10000.0;
        let to = 25.0;

        let mut steinhard =  (res / ro).ln() / beta;
        steinhard = steinhard + (1.0 / (to + 273.15));
        steinhard = (1.0 / steinhard) - 273.15;

        Ok(ResponseTemperature{
            on_board_temp: steinhard,
            pmc_temp: pmc_temp
        })
    }
}

