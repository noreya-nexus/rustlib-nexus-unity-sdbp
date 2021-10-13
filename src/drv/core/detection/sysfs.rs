use std::fs::{File};
use std::io::prelude::*;
use std::path::PathBuf;
use filetime::FileTime;

use super::error::ParseDescriptorError;
use regex::Regex;
use crate::datatypes::*;
use crate::drv::core::detection::error::ParseDescriptorErrorSource;
use std::convert::TryFrom;
use crc::{crc32, Hasher32};

#[allow(non_snake_case)]
mod DescriptorFileNames {
    pub const FS_BOOTLOADER_STATE: &str = "bootloader_state";
    pub const FS_FW_VERSION: &str = "fw_version";
    pub const FS_HW_VERSION: &str = "hw_version";
    pub const FS_MAX_POWER_12V: &str = "max_power_12v";
    pub const FS_MAX_POWER_5V0: &str = "max_power_5v0";
    pub const FS_MAX_POWER_3V3: &str = "max_power_3v3";
    pub const FS_MAX_SCLK_SPEED: &str = "max_sclk_speed";

    pub const FS_PRODUCT_NAME: &str = "product_name";
    pub const FS_VENDOR_PRODUCT_ID: &str = "vendor_product_id";
    pub const FS_VENDOR_NAME: &str = "vendor_name";
    pub const FS_PROTOCOL_VERSION: &str = "protocol_version";

    pub const FS_MAX_FRAME_SIZE: &str = "max_frame_size";
    pub const FS_SERIAL_CODE: &str = "serial_code";
    pub const FS_RID: &str = "rid";
}

fn read_string(file_path: PathBuf) -> Option<String> {
    let tries: u8 = 20;
    for _  in 0..tries {  // In case the resource is busy we need to try several times

        trace!("Read from: {}", file_path.to_str().unwrap());

        let mut file = match File::open(&file_path) {
            Ok(value) => value,
            Err(_e) => {
                trace!("Failed to open file");
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            },
        };

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_value) => (),
            Err(_e) => {
                trace!("{}", format!("Failed to read from file: {}", _e));
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            },
        };


        let zero_pattern: [u8; 1] = [0];

        if content.as_bytes().ends_with(&zero_pattern) {
            content.truncate(content.len() - 1);
        }
        return Some(content);
    };
    error!("Failed to read descriptor after {}", tries);
    return None;
}

pub fn get_kernel_driver_version() -> Result<String,ParseDescriptorError> {

    match read_string(PathBuf::from("/sys/module/sdbpk/version")) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::KernelDriver)),
        Some(_val) => return Ok(_val),
    };
}

pub fn get_vendor_id(path : &PathBuf) -> Result<String,ParseDescriptorError> {

    let _vendor_product_id = match read_string(path.join(DescriptorFileNames::FS_VENDOR_PRODUCT_ID)) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::VendorProductId)),
        Some(_val) => return Ok(_val),
    };
}

pub fn get_timestamp(path : &PathBuf) -> u32 {
    let  meta = std::fs::metadata(path).unwrap();
    FileTime::from_last_modification_time(&meta).nanoseconds()
}

pub fn get_bootloader_state(path : &PathBuf) -> Result<BootloaderState,ParseDescriptorError> {

    match read_string(path.join(DescriptorFileNames::FS_BOOTLOADER_STATE)).and_then(| value |
        BootloaderState::try_from(value.as_str()).ok())
    {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::BootloaderState)),
        Some(_val) => Ok(_val),
    }
}

pub fn getid(path : &PathBuf) -> Result<u32,ParseDescriptorError>{

    let r_max_power_12v = match read_string(path.join(DescriptorFileNames::FS_MAX_POWER_12V)).and_then(| value | value.parse::<u16>().ok()) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::MaxPower12v)),
        Some(_val) => _val,
    };

    let r_max_power_5v0 = match read_string(path.join(DescriptorFileNames::FS_MAX_POWER_5V0)).and_then(| value | value.parse::<u16>().ok()) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::MaxPower5v)),
        Some(_val) => _val,
    };

    let r_max_power_3v3 = match read_string(path.join(DescriptorFileNames::FS_MAX_POWER_3V3)).and_then(| value | value.parse::<u16>().ok()) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::MaxPower3v3)),
        Some(_val) => _val,
    };

    let mut hash_input : Vec<u8> = vec![];
    //let ts = get_timestamp(path);
    hash_input.extend_from_slice(&r_max_power_12v.to_be_bytes());
    hash_input.extend_from_slice(&r_max_power_5v0.to_be_bytes());
    hash_input.extend_from_slice(&r_max_power_3v3.to_be_bytes());
    //hash_input.extend_from_slice(&ts.to_be_bytes());
    let mut digest = crc32::Digest::new(crc32::IEEE);
    digest.write(hash_input.as_slice());

    return Ok(digest.sum32());
}

pub fn get_rid(path : &PathBuf) -> Result<u64,ParseDescriptorError>  {

    let rid = match read_string(path.join(DescriptorFileNames::FS_RID)).and_then(| value | value.parse::<u64>().ok()) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::Rid)),
        Some(_val) => _val,
    };
    Ok(rid)
}


pub fn get_descriptor(path : &PathBuf) -> Result<Descriptor,ParseDescriptorError> {


    let regex = Regex::new("slot([0-9]*)").unwrap();
    let mut cap = regex.captures_iter(path.to_str().unwrap());


    let dev_adr = match cap.next() {

        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::Path)),
        Some(capture) => {
            if capture.len() < 2 {
                return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::Path))
            }

            match capture[1].parse::<u16>() {
                Err(_err) => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::DevAdr)),
                Ok(value) => value,
            }
        },
    };

    let mut result = Descriptor::new(path.clone());
    result.set_adr(dev_adr);


    let r_vendor_product_id = match read_string(path.join(DescriptorFileNames::FS_VENDOR_PRODUCT_ID)) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::VendorProductId)),
        Some(_val) => _val,
    };

    let r_bootloader_state = match read_string(path.join(DescriptorFileNames::FS_BOOTLOADER_STATE)).and_then(| value |
        BootloaderState::try_from(value.as_str()).ok())
    {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::BootloaderState)),
        Some(_val) => _val,
    };

    let r_fw_version = match read_string(path.join(DescriptorFileNames::FS_FW_VERSION)).and_then(| value | AdvancedVersion::from_str(value.as_str()).ok()) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::FwVersion)),
        Some(_val) => _val,
    };

    let r_hw_version = match read_string(path.join(DescriptorFileNames::FS_HW_VERSION)).and_then(| value | Version::from_str(value.as_str()).ok()) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::HwVersion)),
        Some(_val) => _val,
    };

    let r_max_frame_size = match read_string(path.join(DescriptorFileNames::FS_MAX_FRAME_SIZE)).and_then(| value | value.parse::<u16>().ok()) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::MaxFrameSize)),
        Some(_val) => _val,
    };

    let r_max_power_12v = match read_string(path.join(DescriptorFileNames::FS_MAX_POWER_12V)).and_then(| value | value.parse::<u16>().ok()) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::MaxPower12v)),
        Some(_val) => _val,
    };

    let r_max_power_5v0 = match read_string(path.join(DescriptorFileNames::FS_MAX_POWER_5V0)).and_then(| value | value.parse::<u16>().ok()) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::MaxPower5v)),
        Some(_val) => _val,
    };

    let r_max_power_3v3 = match read_string(path.join(DescriptorFileNames::FS_MAX_POWER_3V3)).and_then(| value | value.parse::<u16>().ok()) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::MaxPower3v3)),
        Some(_val) => _val,
    };

    let r_max_sclk_speed = match read_string(path.join(DescriptorFileNames::FS_MAX_SCLK_SPEED)).and_then(| value | value.parse::<u32>().ok()) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::MaxSclkSpeed)),
        Some(_val) => _val,
    };

    let r_product_name = match read_string(path.join(DescriptorFileNames::FS_PRODUCT_NAME)) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::ProductName)),
        Some(_val) => _val,
    };

    let r_protocol_version = match read_string(path.join(DescriptorFileNames::FS_PROTOCOL_VERSION)).and_then(|value| Version::from_str(&value).ok())  {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::ProtocolVersion)),
        Some(_val) => _val,
    };

    let r_vendor_name = match read_string(path.join(DescriptorFileNames::FS_VENDOR_NAME)) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::VendorName)),
        Some(_val) => _val,
    };

    let r_serial_code = match read_string(path.join(DescriptorFileNames::FS_SERIAL_CODE)) {
        None => return Err(ParseDescriptorError::new(ParseDescriptorErrorSource::Serial)),
        Some(_val) => _val,
    };

    let dev_file= PathBuf::from(format!("/dev/slot{}",dev_adr));
    match dev_file.exists() {
        true => (),
        false => return  Err(ParseDescriptorError::new(ParseDescriptorErrorSource::DevAdr)),
    }

    result.set_dev_file(dev_file);
    result.set_bootloader_state(format!("{}",r_bootloader_state));
    result.set_fw_version(r_fw_version);
    result.set_hw_version(r_hw_version);
    result.set_max_frame_size(r_max_frame_size);
    result.set_max_sclk_speed(r_max_sclk_speed);
    result.set_max_power_12v(r_max_power_12v);
    result.set_max_power_5v(r_max_power_5v0);
    result.set_max_power_3v3(r_max_power_3v3);
    result.set_product_name(r_product_name);
    result.set_protocol_version(r_protocol_version);
    result.set_serial(r_serial_code);
    result.set_vendor_name(r_vendor_name);
    result.set_vendor_product_id(r_vendor_product_id);
    result.set_uid(getid(path).unwrap());
    Ok(result)
}


