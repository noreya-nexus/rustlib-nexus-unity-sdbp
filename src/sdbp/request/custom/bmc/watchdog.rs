use super::protocol::*;
use std::io::Error;
use std::io::ErrorKind;

pub struct WatchdogBuilder {
    frame : Vec<u8>
}

impl WatchdogBuilder {

    pub fn new() -> WatchdogBuilder {
        let frame = vec![CLASS_ID,classes::watchdog::ID];
        WatchdogBuilder{frame}
    }

    pub fn timeout(mut self, timeout: u32) -> Result<Vec<u8>,Error> {
        if (timeout < 1000 && timeout != 0) || timeout > 600000  { return Err(Error::new(ErrorKind::Other,"timeout out of range")); }

        if timeout == 0 {
            self.frame.push(classes::watchdog::operation_code::DISABLE_TIMEOUT);
        }
        else {
            self.frame.push(classes::watchdog::operation_code::ENABLE_TIMEOUT);
            self.frame.push((timeout >> 24) as u8);
            self.frame.push((timeout >> 16) as u8);
            self.frame.push((timeout >> 8) as u8);
            self.frame.push((timeout >> 0) as u8);
        }
        Ok(self.frame)
    }

    pub fn set_shutdown_timeout(mut self, timeout: u32) -> Result<Vec<u8>,Error> {
        if timeout < 10000 || timeout > 600000  { return Err(Error::new(ErrorKind::Other,"timeout out of range")); }

        self.frame.push(classes::watchdog::operation_code::SET_SHUTDOWN_TIMEOUT);
        self.frame.push((timeout >> 24) as u8);
        self.frame.push((timeout >> 16) as u8);
        self.frame.push((timeout >> 8) as u8);
        self.frame.push((timeout >> 0) as u8);

        Ok(self.frame)
    }

    pub fn get_timeout(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::watchdog::operation_code::GET_TIMEOUT);
        Ok(self.frame)
    }

    pub fn get_timeout_left(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::watchdog::operation_code::GET_TIME_LEFT);
        Ok(self.frame)
    }

    pub fn get_shutdown_timeout(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::watchdog::operation_code::GET_SHUTDOWN_TIMEOUT);
        Ok(self.frame)
    }

    pub fn get_emergency_mode_state(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::watchdog::operation_code::EMERGENCY_MODE_STATE);
        Ok(self.frame)
    }

    pub fn alive(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::watchdog::operation_code::ALIVE);
        Ok(self.frame)
    }

    pub fn save_config(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::watchdog::operation_code::SAVE_CONFIG);
        Ok(self.frame)
    }

    pub fn sw_shutdown(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::watchdog::operation_code::SW_SHUTDOWN);
        Ok(self.frame)
    }


}