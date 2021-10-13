use crate::sdbp::request::custom::power::protocol::*;
use std::io::Error;
use std::io::ErrorKind;

pub struct TemperatureBuilder {
    frame : Vec<u8>
}

impl TemperatureBuilder {
    pub fn new() -> TemperatureBuilder {
        let frame = vec![CLASS_ID, classes::temperature_control_class::ID];
        TemperatureBuilder { frame }
    }

    pub fn temperature_sensor(mut self) ->  Result<Vec<u8>,Error>{
        self.frame.push(classes::temperature_control_class::operation_code::TEMPERATURE_SENSOR);
        Ok(self.frame)
    }

    pub fn fan_status(mut self) ->  Result<Vec<u8>,Error>{
        self.frame.push(classes::temperature_control_class::operation_code::FAN_STATUS);
        Ok(self.frame)
    }
    pub fn fan_control(mut self,fan_forced : bool, fan_mode : Option<u8>) ->  Result<Vec<u8>,Error>{

        let fan_mode= match fan_mode {
            Some(0) => 0,
            Some(20) => 3,
            Some(40) => 4,
            Some(60) => 5,
            Some(80) => 6,
            Some(100) => 1,
            None => 0,
            _ => return Err(Error::new(ErrorKind::InvalidData, "Fan mode must be 0, 20, 40, 60, 80 ,100")),
        };

        self.frame.push(classes::temperature_control_class::operation_code::FAN_CONTROL);

        if fan_forced {
            self.frame.push(1 as u8);
        } else {
            self.frame.push(2 as u8);
        }
        self.frame.push(fan_mode);
        Ok(self.frame)
    }

    pub fn fan_rpm(mut self) ->  Result<Vec<u8>,Error>{
        self.frame.push(classes::temperature_control_class::operation_code::FAN_RPM);
        Ok(self.frame)
    }

    pub fn fan_rpm_control(mut self,measurement : bool) ->  Result<Vec<u8>,Error>{

        self.frame.push(classes::temperature_control_class::operation_code::FAN_RPM_CONTROL);

        if measurement{
            self.frame.push(1 as u8);
        } else {
            self.frame.push(2 as u8);
        }
        Ok(self.frame)
    }
}
