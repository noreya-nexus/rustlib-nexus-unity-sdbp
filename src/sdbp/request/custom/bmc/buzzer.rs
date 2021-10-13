use super::protocol::*;
use std::io::Error;
use std::io::ErrorKind;

pub struct BuzzerBuilder {
    frame : Vec<u8>
}

impl BuzzerBuilder {

    pub fn new() -> BuzzerBuilder {
        let frame = vec![CLASS_ID,classes::buzzer::ID];
        BuzzerBuilder{frame}
    }

    pub fn buzzer(mut self, mode: u8, duration: u32) -> Result<Vec<u8>,Error> {
        if mode > 5  { return Err(Error::new(ErrorKind::Other,"mode out of range")); }
        if duration > 1000 || duration < 1  { return Err(Error::new(ErrorKind::Other,"duration out of range")); }

        self.frame.push(classes::buzzer::operation_code::MODE_BUZZER);
        self.frame.push(mode);
        self.frame.push((duration >> 24) as u8);
        self.frame.push((duration >> 16) as u8);
        self.frame.push((duration >> 8) as u8);
        self.frame.push((duration >> 0) as u8);
        Ok(self.frame)
    }

}