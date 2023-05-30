use super::protocol::*;
use std::io::Error;
use std::io::ErrorKind;

pub struct UsbHubBuilder {
    frame : Vec<u8>
}

impl UsbHubBuilder {

    pub fn new() -> UsbHubBuilder {
        let frame = vec![CLASS_ID,classes::usbhub::ID];
        UsbHubBuilder{frame}
    }

    pub fn get_hub_state(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::usbhub::operation_code::GET_HUB_STATE);
        Ok(self.frame)
    }

    pub fn get_slot_state(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::usbhub::operation_code::GET_USB_SLOT_STATE);
        Ok(self.frame)
    }

    pub fn get_port_mapping(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::usbhub::operation_code::GET_PORT_MAPPING);
        Ok(self.frame)
    }

    pub fn set_hub_state(mut self, state: bool) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::usbhub::operation_code::SET_HUB_STATE);
        if state {
            self.frame.push(0x01);
        }
        else {
            self.frame.push(0x02);
        }
        Ok(self.frame)
    }

    pub fn set_slot_state(mut self, state: bool, number: u8) -> Result<Vec<u8>,Error> {
        if number < 2 || number > 8 { return Err(Error::new(ErrorKind::Other,"port out of range")); }

        self.frame.push(classes::usbhub::operation_code::SET_USB_SLOT_STATE);
        if state {
            self.frame.push(0x01);
        }
        else {
            self.frame.push(0x02);
        }
        self.frame.push(number);
        Ok(self.frame)
    }

    pub fn hub_reset(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::usbhub::operation_code::HUB_RESET);
        Ok(self.frame)
    }
}