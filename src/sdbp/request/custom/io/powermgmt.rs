use super::protocol::*;
use std::io::Error;
use std::io::ErrorKind;

pub struct PowerMgmtBuilder {
    frame : Vec<u8>
}

impl PowerMgmtBuilder {

    pub fn new() -> PowerMgmtBuilder {
        let frame = vec![CLASS_ID,classes::power_management_class::ID];
        PowerMgmtBuilder {frame}
    }

    pub fn set_power_config(mut self,pin_config : Vec<(u8,u16)>) -> Result<Vec<u8>,Error> {
        if pin_config.len() != 6  {
            return Err(Error::new(ErrorKind::Other,"Invalid pin configuration"));
        }

        self.frame.push(classes::power_management_class::operation_code::SET_POWER_CONFIG);

       for i in 0..pin_config.len() {

           match pin_config[i].0 {
               0 | 1 => (),
               _ => return Err(Error::new(ErrorKind::Other,format!("Invalid voltage rail for pin: {}",i))),
           }

           if pin_config[i].0 == 0 {
               match pin_config[i].1 {
                   0..=300 => (),
                   _ => return Err(Error::new(ErrorKind::Other, format!("Invalid current limit for pin: {}", i))),
               }
           }

           if pin_config[i].0  == 1 {
               match pin_config[i].1 {
                   0..=500 => (),
                   _ => return Err(Error::new(ErrorKind::Other, format!("Invalid current limit for pin: {}", i))),
               }
           }
           self.frame.push(pin_config[i].0);
           self.frame.push((pin_config[i].1 >> 8) as u8);
           self.frame.push((pin_config[i].1 >> 0) as u8);
       }

        Ok(self.frame)
    }

    pub fn test_power_config(mut self,pin_config : Vec<(u8,u16)>) -> Result<Vec<u8>,Error> {
        if pin_config.len() != 6  {
            return Err(Error::new(ErrorKind::Other,"Invalid pin configuration"));
        }

        self.frame.push(classes::power_management_class::operation_code::TEST_POWER_CONFIG);

        for i in 0..pin_config.len() {

            match pin_config[i].0 {
                0 | 1 => (),
                _ => return Err(Error::new(ErrorKind::Other,format!("Invalid voltage rail for pin: {}",i))),
            }

            if pin_config[i].0  == 0 {
                match pin_config[i].1 {
                    0..=300 => (),
                    _ => return Err(Error::new(ErrorKind::Other, format!("Invalid current limit for pin: {}", i))),
                }
            }

            if pin_config[i].0  == 1 {
                match pin_config[i].1 {
                    0..=500 => (),
                    _ => return Err(Error::new(ErrorKind::Other, format!("Invalid current limit for pin: {}", i))),
                }
            }
            self.frame.push(pin_config[i].0);
            self.frame.push((pin_config[i].1 >> 8)as u8);
            self.frame.push((pin_config[i].1 >> 0)as u8);
        }

        Ok(self.frame)
    }
}

