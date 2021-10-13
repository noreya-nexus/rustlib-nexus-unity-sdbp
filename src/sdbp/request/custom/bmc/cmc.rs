use std::io::Error;
use std::io::ErrorKind;

use super::protocol::*;

pub struct CmcBuilder {
    frame: Vec<u8>
}

impl CmcBuilder {
    pub fn new() -> CmcBuilder {
        let frame = vec![CLASS_ID, classes::cmc::ID];
        CmcBuilder { frame }
    }

    pub fn set_usb_bootloader(mut self, enable: bool, timeout: u32) -> Result<Vec<u8>, Error> {
        if timeout > 3600000 { return Err(Error::new(ErrorKind::Other, "timeout out of range")); }

        self.frame.push(classes::cmc::operation_code::CTL_USBBOOT);
        if enable {
            self.frame.push(0x01);
        } else {
            self.frame.push(0x02);
        }
        self.frame.push((timeout >> 24) as u8);
        self.frame.push((timeout >> 16) as u8);
        self.frame.push((timeout >> 8) as u8);
        self.frame.push((timeout >> 0) as u8);

        Ok(self.frame)
    }

    pub fn hard_reset(mut self) -> Result<Vec<u8>, Error> {
        self.frame = vec![classes::control::CLASS_ID, classes::control::ID, classes::control::operation_code::SYSTEM_RESET];  // Core SYSTEM_RESET command
        Ok(self.frame)
    }
}