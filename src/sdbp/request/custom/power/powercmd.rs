use std::io::Error;
use std::io::ErrorKind;

use super::protocol::*;
use crate::sdbp::request::custom::power::protocol::classes;


pub struct PowerBuilder {
    frame : Vec<u8>
}

impl PowerBuilder {

    pub fn new() -> PowerBuilder {
        let frame = vec![CLASS_ID,classes::power_class::ID];
        PowerBuilder{frame}
    }

    pub fn source(mut self) -> Result<Vec<u8>,Error> {
        self.frame.push(classes::power_class::operation_code::SOURCE);
        Ok(self.frame)
    }

    pub fn current_limit(mut self, limit_3v3 : u32, limit_5v0: u32, limit_12v: u32) -> Result<Vec<u8>,Error> {

        if limit_3v3 < 1000 || limit_3v3 > 5950 {
            return Err(Error::new(ErrorKind::Other,"Valid limit_3v3 range 1000 - 5950"));
        }

        if limit_5v0 < 1000 || limit_5v0 > 4950 {
           return Err(Error::new(ErrorKind::Other,"Valid limit_5v0 range 1000 - 4950"));
        }

        if limit_12v < 100 || limit_12v > 2400 {
            return Err(Error::new(ErrorKind::Other,"Valid limit_12v range 100 - 2400 "));
        }

        self.frame.push(classes::power_class::operation_code::CURRENT_LIMIT);

        self.frame.push((limit_3v3 >> 24) as u8);
        self.frame.push((limit_3v3 >> 16) as u8);
        self.frame.push((limit_3v3 >> 8) as u8);
        self.frame.push((limit_3v3 >> 0) as u8);

        self.frame.push((limit_5v0 >> 24) as u8);
        self.frame.push((limit_5v0 >> 16) as u8);
        self.frame.push((limit_5v0 >> 8) as u8);
        self.frame.push((limit_5v0 >> 0) as u8);

        self.frame.push((limit_12v >> 24) as u8);
        self.frame.push((limit_12v >> 16) as u8);
        self.frame.push((limit_12v >> 8) as u8);
        self.frame.push((limit_12v >> 0) as u8);
        Ok(self.frame)
    }

    pub fn voltage_current_status(mut self) -> Result<Vec<u8>,Error>{
        self.frame.push(classes::power_class::operation_code::VOLTAGE_CURRENT_STATUS);
        Ok(self.frame)
    }

    pub fn protection_status(mut self ) -> Result<Vec<u8>,Error>{
        self.frame.push(classes::power_class::operation_code::OPP_CNT_STATUS);
        Ok(self.frame)
    }

}

#[allow(unused_imports,unused_macros)]
mod test {
    use super::*;

    macro_rules! assert_result {
        ($result:expr) => {
            match $result {
                Ok(value) => value,
                Err(err) => panic!(format!("{}", err)),
            }
        };
    }

    #[test]
    fn test_source() {
        let expected = vec![CLASS_ID,classes::power_class::ID,classes::power_class::operation_code::SOURCE];
        let result = assert_result!(PowerBuilder::new().source());
        assert_eq!(expected,result,"Failed to build command SOURCE")
    }

    #[test]
    fn test_current_limit() {
        let expected = vec![CLASS_ID,classes::power_class::ID,classes::power_class::operation_code::CURRENT_LIMIT,
        0x00,0x00,0x03,0xE8,
        0x00,0x00,0x03,0xE8,
        0x00,0x00,0x00,0x64];
        let result = assert_result!(PowerBuilder::new().current_limit(1000,1000,100));
        assert_eq!(expected,result,"Failed to build command CURRENT_LIMIT")
    }

    #[test]
    #[should_panic]
    fn test_current_limit_err1() {
        let expected = vec![CLASS_ID,classes::power_class::ID,classes::power_class::operation_code::CURRENT_LIMIT,
                            0x00,0x00,0x03,0xE8,
                            0x00,0x00,0x03,0xE8,
                            0x00,0x00,0x00,0x64];
        let result = assert_result!(PowerBuilder::new().current_limit(999,1000,100));
        assert_eq!(expected,result,"Failed to build command CURRENT_LIMIT")
    }

    #[test]
    fn voltage_current_status() {
        let expected = vec![CLASS_ID,classes::power_class::ID,classes::power_class::operation_code::VOLTAGE_CURRENT_STATUS];
        let result = assert_result!(PowerBuilder::new().voltage_current_status());
        assert_eq!(expected,result,"Failed to build command VOLTAGE_CURRENT_STATUS")
    }
}
