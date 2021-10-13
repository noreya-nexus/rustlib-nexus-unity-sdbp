use super::*;
use std::fmt;
use std::path::{PathBuf};
use std::convert::TryFrom;

pub struct BootloaderStateErr;
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BootloaderState{
    NotSupported,
    Supported,
    BootloaderMode,
    NotInitialized
}

impl Default for BootloaderState {
    fn default() -> Self {
      BootloaderState::NotInitialized
    }
}

impl TryFrom<u8> for BootloaderState {
    type Error = BootloaderStateErr;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BootloaderState::NotSupported),
            1 => Ok(BootloaderState::Supported),
            2 => Ok(BootloaderState::BootloaderMode),
            _ => Err(BootloaderStateErr)
        }
    }
}

impl TryFrom<&str> for BootloaderState {
    type Error = BootloaderStateErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0" => Ok(BootloaderState::NotSupported),
            "1" => Ok(BootloaderState::Supported),
            "2" => Ok(BootloaderState::BootloaderMode),
            "not supported" => Ok(BootloaderState::NotSupported),
            "supported" => Ok(BootloaderState::Supported),
            "in bootloader mode" => Ok(BootloaderState::BootloaderMode),
            _ => Err(BootloaderStateErr)
        }
    }
}


impl fmt::Display for BootloaderState {

    fn fmt(&self, fmt : &mut fmt::Formatter) -> fmt::Result {

        match self {
            BootloaderState::NotSupported =>  fmt.write_str(fmt::format(format_args!("not supported")).as_str()).unwrap(),
            BootloaderState::Supported =>  fmt.write_str(fmt::format(format_args!("supported")).as_str()).unwrap(),
            BootloaderState::BootloaderMode =>  fmt.write_str(fmt::format(format_args!("in bootloader mode")).as_str()).unwrap(),
            BootloaderState::NotInitialized =>  fmt.write_str(fmt::format(format_args!("not initialized")).as_str()).unwrap(),
            //_ => fmt.write_str(fmt::format(format_args!("not supported (unknown)")).as_str()).unwrap(),
        }
        Ok(())
    }
}


#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Descriptor {

    #[serde(skip_serializing)]
    path: PathBuf,
    slot_number: u16,
    vendor_product_id:String,
    product_name:String,
    vendor_name:String,
    serial_code:String,
    fw_version:AdvancedVersion,
    hw_version:Version,
    protocol_version: Version,
    bootloader_state:String,
    max_frame_size:u16,
    max_power_12v:u16,
    max_power_5v0:u16,
    max_power_3v3:u16,
    max_sclk_speed:u32,

    #[serde(skip_serializing)]
    timestamp:u32,

    #[serde(skip_serializing)]
    dev_file: PathBuf,

    #[serde(skip_serializing)]
    uid : u32,

}

#[allow(dead_code)]
impl Descriptor {

    pub fn new(_path : PathBuf) -> Descriptor {

        Descriptor {
            path: _path.clone(),
            slot_number: 0,
            bootloader_state: "".to_string(),
            fw_version: Default::default(),
            hw_version: Default::default(),
            max_frame_size: 0,
            max_power_12v: 0,
            max_power_5v0: 0,
            max_power_3v3: 0,
            max_sclk_speed: 0,
            product_name: "".to_string(),
            protocol_version: Default::default(),
            serial_code: "".to_string(),
            vendor_name: "".to_string(),
            vendor_product_id: "".to_string(),
            timestamp:0,
            dev_file : _path,
            uid : 0,
        }
    }


    pub fn path(&self) -> &PathBuf {&self.path}
    pub fn adr(&self) -> u16 { self.slot_number }
    pub fn set_adr(&mut self, adr : u16) {
        self.slot_number = adr;
    }

    pub fn bootloader_state(&self) -> String { self.bootloader_state.clone()}
    pub fn set_bootloader_state(&mut self, state : String) {
        self.bootloader_state = state;
    }

    pub fn fw_version(&self) -> &AdvancedVersion {&self.fw_version}
    pub fn set_fw_version(&mut self, version : AdvancedVersion) {
        self.fw_version = version;
    }

    pub fn hw_version(&self) -> &Version {&self.hw_version}
    pub fn set_hw_version(&mut self,version : Version) {
        self.hw_version = version;
    }

    pub fn max_frame_size(&self) -> u16 {self.max_frame_size}
    pub fn set_max_frame_size(&mut self, frame_size : u16) {
        self.max_frame_size = frame_size;
    }

    pub fn max_sclk_speed(&self) -> u32 {self.max_sclk_speed}
    pub fn set_max_sclk_speed(&mut self, speed : u32) {
        self.max_sclk_speed = speed;
    }

    pub fn max_power_5v(&self) -> u16 {self.max_power_5v0}
    pub fn set_max_power_5v(&mut self, power : u16) {
        self.max_power_5v0 = power;
    }

    pub fn max_power_3v3(&self) -> u16 {self.max_power_3v3}
    pub fn set_max_power_3v3(&mut self, power : u16) {
        self.max_power_3v3 = power;
    }
    pub fn max_power_12v(&self) -> u16 {self.max_power_12v}
    pub fn set_max_power_12v(&mut self, power : u16) {
        self.max_power_12v = power;
    }

    pub fn product_name(&self) -> &String {&self.product_name}
    pub fn set_product_name(&mut self, name : String) {
        self.product_name = name;
    }

    pub fn protocol_version(&self) -> &Version {&self.protocol_version}
    pub fn set_protocol_version(&mut self, version : Version) {
        self.protocol_version = version;
    }

    pub fn serial (&self) -> &String {&self.serial_code}
    pub fn set_serial(&mut self,serial : String ) {
        self.serial_code = serial;
    }

    pub fn vendor_name(&self) -> &String {&self.vendor_name}
    pub fn set_vendor_name(&mut self, name : String ) {
        self.vendor_name = name;
    }

    pub fn vendor_product_id(&self) -> &String { &self.vendor_product_id}
    pub fn set_vendor_product_id(&mut self, product_id : String ) {
        self.vendor_product_id = product_id;
    }

    pub fn timestamp(&self) -> u32 { self.timestamp }
    pub fn set_timestamp(&mut self,timestamp : u32) {
        self.timestamp = timestamp;
    }

    pub fn dev_file(&self) -> &PathBuf { &self.dev_file }
    pub fn set_dev_file(&mut self,path: PathBuf) {self.dev_file = path; }

    pub fn uid(&self) -> u32 { self.uid }
    pub fn set_uid(&mut self,uid : u32) {self.uid= uid; }
}

impl fmt::Display for Descriptor {

    fn fmt(&self, fmt : &mut fmt::Formatter) -> fmt::Result {

        fmt.write_str(fmt::format(format_args!("Slot: {}\n",self.path.to_str().unwrap())).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Device Address: {}\n",self.slot_number)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Product Name: {}\n",self.product_name)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Vendor Name: {}\n",self.vendor_name)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Vendor Product ID: {}\n",self.vendor_product_id)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Bootloader State: {}\n",self.bootloader_state)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Firmware Version: {}\n",self.fw_version)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Hardware Version: {}\n",self.hw_version)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Max Frame Size: {}\n",self.max_frame_size)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Max Power 12V: {}\n",self.max_power_12v)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Max Power 5V : {}\n",self.max_power_5v0)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Max Power 3V3: {}\n",self.max_power_3v3)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Max SCLK Speed: {}\n",self.max_sclk_speed)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Serial Code {}\n",self.serial_code)).as_str()).unwrap();
        fmt.write_str(fmt::format(format_args!("Protocol Version {}\n",self.protocol_version)).as_str()).unwrap();
        Ok(())
    }
}
