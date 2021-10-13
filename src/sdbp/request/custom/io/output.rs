use super::protocol::*;
use std::io::Error;
use std::io::ErrorKind;

pub struct OutputBuilder {
    frame : Vec<u8>
}

impl OutputBuilder {

    pub fn new() -> OutputBuilder {
        let frame = vec![CLASS_ID,classes::output_class::ID];
        OutputBuilder{frame}
    }

    pub fn set_output(mut self, pin_nr : u8, mode : u8, state: u8 ) -> Result<Vec<u8>,Error> {

        match mode {
            1 => (),
            _ => return Err(Error::new(ErrorKind::Other,"Invalid input mode")),
        }

        match state {
            0 | 1 => (),
            _ => return Err(Error::new(ErrorKind::Other,"Invalid state")),
        }

        self.frame.push(classes::output_class::operation_code::SET_OUTPUT);
        self.frame.push(pin_nr);
        self.frame.push(mode);
        self.frame.push(state);
        Ok(self.frame)
    }

    pub fn set_output_pwm(mut self, pin_nr : u8, mode : u8, prescaler: u16, time_on : u32, period : u32 ) -> Result<Vec<u8>,Error> {

        match mode {
             2 => (),
            _ => return Err(Error::new(ErrorKind::Other,"Invalid input mode")),
        }

        self.frame.push(classes::output_class::operation_code::SET_OUTPUT);
        self.frame.push(pin_nr);
        self.frame.push(mode);
        self.frame.push((prescaler >> 8) as u8);
        self.frame.push((prescaler >> 0) as u8);
        self.frame.push((time_on >> 24) as u8);
        self.frame.push((time_on >> 16) as u8);
        self.frame.push((time_on >> 8) as u8);
        self.frame.push((time_on >> 0) as u8);
        self.frame.push((period >> 24) as u8);
        self.frame.push((period >> 16) as u8);
        self.frame.push((period >> 8) as u8);
        self.frame.push((period >> 0) as u8);
        Ok(self.frame)
    }




}

