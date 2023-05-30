use std::path::PathBuf;

use std::time::Duration;
use std::io::{Error, ErrorKind};
use std::os::unix::net::UnixStream;

use crate::sdbp::response::SdbpResponse;
use crate::drv::api::{FrameBuilder, Tag, TlvValue, Response};
use crate::util::{UnixStreamReader, Connection};
use crate::datatypes::{Version, Descriptor};
use std::convert::TryFrom;
use crate::datatypes::BootloaderState;

pub struct Manager {
   com : UnixStreamReader,
   is_selected : bool,
    selected_slot : u16,
}


#[derive(Clone)]
pub struct ModApiInfo {
    drv_version: Version,
    sdbpk_drv_version: Version,
}


impl ModApiInfo {

    pub fn from(drv_version : Version, sdbpk_drv_version: Version) -> ModApiInfo {
        ModApiInfo { drv_version, sdbpk_drv_version}
    }

    pub fn get_version(self) -> Version {
        self.drv_version
    }
    pub fn get_sdbpk_version(self) -> Version {
        self.sdbpk_drv_version
    }

}



#[allow(unused)]
impl Manager {

    pub fn new(socket_path : String, timeout : Option<Duration>) -> Result<Manager,Error> {

        let stream = match UnixStream::connect(socket_path) {
            Ok(value) => value,
            Err(err) =>{
                trace!("{}",err);
                return Err(err);
            }
        };
        Ok(Manager{com : UnixStreamReader::from_unix_stream(stream,timeout), is_selected : false, selected_slot : 0})
    }

    pub fn select_via_slot(&mut self, slot : u16) -> Result<(),Error>{


        if slot & 0x2000 == 0 {
            let devices = match self.get_device_list(true) {
                Ok(value) => value,
                Err(err) => return Err(err),
            };

            for device in devices {
                if device.adr() == slot {
                    self.selected_slot = device.adr();
                    self.is_selected = true;
                    return Ok(());
                }
            }
            return Err(Error::new(ErrorKind::NotFound, format!("Cannot find device with address {}", slot)));
        }
        self.selected_slot = slot;
        self.is_selected = true;
        return Ok(());

    }

    pub fn select_via_descriptor(&mut self, desc : &Descriptor) -> Result<(),Error> {
        let devices  =  match self.get_device_list(true){
            Ok(value) => value,
            Err(err)  => return Err(err),
        };

        for  device in devices {
            if device.adr() == desc.adr() {
                self.selected_slot = device.adr();
                self.is_selected = true;
                return Ok(());
            }
        }
        return Err(Error::new(ErrorKind::NotFound,format!("Cannot find device with address {}",desc.adr())));
    }

    pub fn select_via_serial(&mut self, serial : String) -> Result<(),Error> {
        let devices  =  match self.get_device_list(true){
            Ok(value) => value,
            Err(err)  => return Err(err),
        };

        for  device in devices {
            if *device.serial() == serial  {
                self.selected_slot = device.adr();
                self.is_selected = true;
                return Ok(());
            }
        }
        return Err(Error::new(ErrorKind::NotFound,format!("Cannot find device with serial {}",serial)));
    }


    pub fn get_info(&mut self) -> Result<ModApiInfo,Error> {

        match self.com.write_msg(FrameBuilder::request().info().to_bytes())     {
            Ok(_value) => (),
            Err(err) => return Err(err),
        }

        let raw = match self.com.read_msg() {
            Ok(value) => value,
            Err(err) => return Err(err)
        };

        let response = match Response::from_bytes(raw.as_slice()) {
            Some(value) => value,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"Response invalid")),
        };

        let tlv = match TlvValue::try_from(response.get_payload()) {
            Ok(value ) => value,
            Err(_err)  =>  {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"TLV Parsing failed (get_info)."))
            },
        };

        Ok(ModApiInfo{ drv_version : tlv[Tag::InfoBlock][Tag::DriverVersion].as_version().unwrap().clone(), sdbpk_drv_version: tlv[Tag::InfoBlock][Tag::SdbpkDriverVersion].as_version().unwrap().clone() })
    }

    pub fn get_device_list(&mut self,option : bool) -> Result<Vec<Descriptor>,std::io::Error> {

        let mut result = Vec::new();

        match self.com.write_msg(FrameBuilder::request().get_device_list(option).to_bytes()) {
            Ok(_value) => (),
            Err(err) => return Err(err),
        }

        let raw = match self.com.read_msg() {
            Ok(value) => value,
            Err(err) => return Err(err)
        };

        let response = match Response::from_bytes(raw.as_slice()) {
            Some(value) => value,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"Response invalid")),
        };

        let tlv = match TlvValue::try_from(response.get_payload()) {
            Ok(value ) => value,
            Err(_err)  =>  {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"TLV Parsing failed (get_device_list)."))
            },
        };

        for block in tlv.members() {
            if block.0 == Tag::DeviceBlock {
                let mut desc = Descriptor::new(PathBuf::new());
                for (tag, value) in block.1.members() {
                    match tag {

                        Tag::DeviceAddress => desc.set_adr(value.as_u16().unwrap()),
                        Tag::BootloaderState => {
                            let bl_state = match  BootloaderState::try_from(value.as_string().unwrap().to_owned().as_str()) {
                                Ok(value) => {value},
                                Err(_) => panic!("Could not parse bootloader state!"),
                            };
                            let bl_state = format!("{}", bl_state);
                            desc.set_bootloader_state(bl_state)
                        },
                        Tag::HardwareVersion => desc.set_hw_version(value.as_version().unwrap().clone()),
                        Tag::FirmwareVersion => desc.set_fw_version(value.as_advanced_version().unwrap().clone()),
                        Tag::SupportedSdbpVersion => desc.set_protocol_version(value.as_version().unwrap().clone()),
                        Tag::MaxFrameSize => desc.set_max_frame_size(value.as_u16().unwrap()),
                        Tag::SerialNumber => desc.set_serial(value.as_string().unwrap().clone()),
                        Tag::ProductName => desc.set_product_name(value.as_string().unwrap().clone()),
                        Tag::VendorName => desc.set_vendor_name(value.as_string().unwrap().clone()),
                        Tag::MaxPower3v3 => desc.set_max_power_3v3(value.as_u16().unwrap()),
                        Tag::MaxPower5v => desc.set_max_power_5v(value.as_u16().unwrap()),
                        Tag::MaxPower12v => desc.set_max_power_12v(value.as_u16().unwrap()),
                        Tag::MaxSclkSpeed => desc.set_max_sclk_speed(value.as_32().unwrap()),
                        Tag::VendorProductId => desc.set_vendor_product_id(value.as_string().unwrap().clone()),
                        Tag::DeviceSession => desc.set_device_session(value.as_string().unwrap().clone()),
                        _ => (),
                    }
                }
                result.push(desc);
            }
        }
        return Ok(result);
    }



    pub fn raw_command(&mut self, raw_command : Vec<u8>) -> Result<TlvValue,std::io::Error> {

        if !self.is_selected {
            return Err(Error::new(ErrorKind::AddrNotAvailable,"Device is not selected"));
        }

        let request = FrameBuilder::request().device_command(self.selected_slot,raw_command.as_slice());

        match self.com.write_msg(request.to_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        let raw_response = match self.com.read_msg() {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        let response = match Response::from_bytes(raw_response.as_slice()) {
            Some(value) => value,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"Response invalid")),
        };

        let tlv = match TlvValue::try_from(response.get_payload()) {
            Ok(value ) => value,
            Err(_err)  =>  {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"TLV Parsing failed (raw_command)."))
            },
        };

        return Ok(tlv);
    }


    pub fn device_command<T>(&mut self, raw_command : Vec<u8>) -> Result<T,std::io::Error> where  T : SdbpResponse  {

        if !self.is_selected {
            return Err(Error::new(ErrorKind::AddrNotAvailable,"Device is not selected"));
        }

        let request = FrameBuilder::request().device_command(self.selected_slot,raw_command.as_slice());

        match self.com.write_msg(request.to_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        let raw_response = match self.com.read_msg() {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        let response = match Response::from_bytes(raw_response.as_slice()) {
            Some(value) => value,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"Response invalid")),
        };

        let tlv = match TlvValue::try_from(response.get_payload()) {
            Ok(value ) => value,
            Err(_err)  =>  {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"TLV Parsing failed (device_command)."))
            },
        };

        T::from_raw(tlv[Tag::Response].as_bytes().unwrap().clone())
    }

    pub fn get_descriptor(&mut self, device : &mut Descriptor, short : bool) -> Result<(),std::io::Error>{

        let request = FrameBuilder::request().get_descriptor(short, device.adr());


        match self.com.write_msg(request.to_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
        let raw_response = match self.com.read_msg() {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        let response = match Response::from_bytes(raw_response.as_slice()) {
            Some(value) => value,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"Response invalid")),
        };

        let tlv = match TlvValue::try_from(response.get_payload()) {
            Ok(value ) => value,
            Err(_err)  =>  return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"TLV Parsing failed (get_descriptor).")),
        };




        Ok(())
    }
}



