use super::protocol::*;
use std::io::{Error, ErrorKind};

pub struct ControlBuilder {
    frame: Vec<u8>,
}

impl ControlBuilder {

    pub fn new() -> ControlBuilder{
        let mut frame = vec![];
        frame.push(CLASS_ID); //Core
        frame.push(classes::control::ID); //Control Class
        ControlBuilder{
            frame
        }
    }
    pub fn mode_suspend(mut self) -> Result<Vec<u8>,Error>{
        self.frame.push(classes::control::operation_code::MODE_SUSPEND);
        Ok(self.frame)    }

    pub fn mode_run(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::control::operation_code::MODE_RUN);
        Ok(self.frame)    }

    pub fn mode_bootloader(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::control::operation_code::MODE_BOOTLOADER);
        Ok(self.frame)    }

    pub fn system_reset(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::control::operation_code::SYSTEM_RESET);
        Ok(self.frame)    }

    pub fn factory_reset(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::control::operation_code::FACTORY_RESET);
        Ok(self.frame)    }

    pub fn set_frame_size(mut self,size: u16) -> Result<Vec<u8>,Error> {

        if size < 64 || size > 65472 {
            return Err(Error::new(ErrorKind::Other, "Invalid frame size range 64 - 65472"));
        }

        self.frame.push(classes::control::operation_code::SET_FRAME_SIZE);
        self.frame.push((size >> 8) as u8);
        self.frame.push((size) as u8);
        Ok(self.frame)    }

    pub fn set_sclk_speed(mut self,size: u32) -> Result<Vec<u8>,Error>{
        self.frame.push(classes::control::operation_code::SET_SCLK_SPEED);
        self.frame.push((size >> 24) as u8);
        self.frame.push((size >> 16) as u8);
        self.frame.push((size >> 8) as u8);
        self.frame.push((size) as u8);
        Ok(self.frame)    }

    pub fn update_descriptor(mut self) -> Result<Vec<u8>,Error>{
        self.frame.push(classes::control::operation_code::UPDATE_DESCRIPTOR);
        Ok(self.frame)    }
}

#[allow(unused_imports,unused_macros)]
mod test {

    use super::*;

    macro_rules! assert_result {
        ($result:expr) => {
            match $result {
                Ok(value) => value,
                Err(err) => panic!("{}", err),
            }
        };
    }

    #[test]
    fn test_cmd_mode_suspend() {

        let expected : Vec<u8> = vec![CLASS_ID, classes::control::ID, classes::control::operation_code::MODE_SUSPEND];
        let result = assert_result!(ControlBuilder::new().mode_suspend());
        assert_eq!(expected, result,"Failed to build MODE_SUSPEND")
    }

    #[test]
    fn test_cmd_mode_run() {

        let expected : Vec<u8> = vec![CLASS_ID, classes::control::ID, classes::control::operation_code::MODE_RUN];
        let result =  assert_result!(ControlBuilder::new().mode_run());
        assert_eq!(expected, result,"Failed to build MODE_RUN")
    }

    #[test]
    fn test_cmd_mode_bootloader() {

        let expected : Vec<u8> = vec![CLASS_ID, classes::control::ID, classes::control::operation_code::MODE_BOOTLOADER];
        let result =  assert_result!(ControlBuilder::new().mode_bootloader());
        assert_eq!(expected, result,"Failed to build MODE_BOOTLOADER")
    }

    #[test]
    fn test_cmd_mode_system_reset() {

        let expected : Vec<u8> = vec![CLASS_ID, classes::control::ID, classes::control::operation_code::SYSTEM_RESET];
        let result =  assert_result!(ControlBuilder::new().system_reset());
        assert_eq!(expected, result,"Failed to build SYSTEM_RESET")
    }

    #[test]
    fn test_cmd_mode_factory_reset() {

        let expected : Vec<u8> = vec![CLASS_ID, classes::control::ID, classes::control::operation_code::FACTORY_RESET];
        let result =  assert_result!(ControlBuilder::new().factory_reset());
        assert_eq!(expected, result,"Failed to build FACTORY_RESET")
    }

    #[test]
    fn test_cmd_set_frame_size() {

        let expected : Vec<u8> = vec![CLASS_ID, classes::control::ID, classes::control::operation_code::SET_FRAME_SIZE,0x00,0xFF];
        let result =   assert_result!(ControlBuilder::new().set_frame_size(0xff));
        assert_eq!(expected, result,"Failed to build SET_FRAME_SIZE")
    }

    #[test]
    #[should_panic]
    fn test_cmd_set_frame_size_err1() {

        let expected : Vec<u8> = vec![CLASS_ID, classes::control::ID, classes::control::operation_code::SET_FRAME_SIZE,0x00,0x00];
        let result =   assert_result!(ControlBuilder::new().set_frame_size(0x00));
        assert_eq!(expected, result,"Failed to build SET_FRAME_SIZE")
    }

    #[test]
    #[should_panic]
    fn test_cmd_set_frame_size_err2() {

        let expected : Vec<u8> = vec![CLASS_ID, classes::control::ID, classes::control::operation_code::SET_FRAME_SIZE,0xFF,0xFF];
        let result =   assert_result!(ControlBuilder::new().set_frame_size(0xffff));
        assert_eq!(expected, result,"Failed to build SET_FRAME_SIZE")
    }

    #[test]
    fn test_cmd_set_sclk_speed() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::control::ID, classes::control::operation_code::SET_SCLK_SPEED,0xAB,0xBA,0xAF,0xFE];
        let result =   assert_result!(ControlBuilder::new().set_sclk_speed(0xABBAAFFE));
        assert_eq!(expected, result,"Failed to build SET_FRAME_SIZE")
    }

    #[test]
    fn test_cmd_update_descriptor() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::control::ID, classes::control::operation_code::UPDATE_DESCRIPTOR];
        let result =   assert_result!(ControlBuilder::new().update_descriptor());
        assert_eq!(expected, result,"Failed to build UPDATE_DESCRIPTOR")
    }
}