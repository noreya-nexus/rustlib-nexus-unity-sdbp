use super::protocol::*;
use std::io::Error;
use std::io::ErrorKind;

pub struct VoltageBuilder {
    frame : Vec<u8>
}

impl VoltageBuilder {

    pub fn new() -> VoltageBuilder {
        let frame = vec![CLASS_ID,classes::input::ID];
        VoltageBuilder{frame}
    }

    pub fn input(mut self, input: u8) -> Result<Vec<u8>,Error> {
        if input < classes::input::operation_code::input::NTC_0 || input > classes::input::operation_code::input::NTC_1 {
            return Err(Error::new(ErrorKind::Other,"input out of range"));
        }
        self.frame.push(classes::input::operation_code::GET_VOLTAGE);
        self.frame.push(input);
        Ok(self.frame)
    }

}