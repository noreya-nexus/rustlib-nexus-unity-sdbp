use datatypes::*;

use std::fs::{File};
use std::io::prelude::*;
use std::path::{PathBuf, Path};
use std::fmt;

use filetime::FileTime;
use regex::Regex;


#[derive(Debug, Clone,PartialEq)]
enum DescriptorErrorKind {
    ParsingError,
}

#[derive(Debug, Clone,PartialEq)]
pub struct DescriptorError {
    attribute: &'static str,
    kind: DescriptorErrorKind,
}

fn read_raw(file_path: PathBuf) -> Option<String> {

    trace!("Read from: {}",file_path.to_str().expect("Could not read from path"));

    let mut file = match  File::open(file_path) {
        Ok(value) => value,
        Err(_e) => {
            error!("This should not happen");
            return None},
    };

    let mut content = String::new();
    match file.read_to_string(&mut content){
        Ok(_value) => (),
        Err(_e) => return None,
    };


    let zero_pattern : [u8;1] = [0];

    if content.as_bytes().ends_with( &zero_pattern) {
        content.truncate(content.len()-1);
    }
    Some(content)
}



pub fn get_descriptor(path : &PathBuf) -> Opti{


}


pub fn get_vendor_id(path : &PathBuf) -> Result<String,DescriptorError> {

    let _vendor_product_id = match read_raw(path.join(attributes::FS_VENDOR_PRODUCT_ID)) {
        None => return Err(DescriptorError { attribute : attributes::FS_VENDOR_PRODUCT_ID, kind : DescriptorErrorKind::ParsingError}),
        Some(_val) => return Ok(_val),
    };
}


pub fn get_timestamp(path : &PathBuf) -> u32 {
    let  meta = std::fs::metadata(path).expect("Could not get timestamp");
    FileTime::from_last_modification_time(&meta).nanoseconds()
}



// #[allow(dead_code)]
// impl FromSysFS for Descriptor {
//
//
//
//
//
//     pub fn update(& mut self, parent_dir: &Path, slot_nr : &String) -> Result<(),DescriptorError>{
//         let path = PathBuf::from(parent_dir).join(slot_nr);
//
//         let r_vendor_product_id = match read_raw(path.join(attributes::FS_VENDOR_PRODUCT_ID)) {
//             None => return Err(DescriptorError { attribute : attributes::FS_VENDOR_PRODUCT_ID, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_bootloader_state = match read_raw(path.join(attributes::FS_BOOTLOADER_STATE)).and_then(| value | match value.as_str() {
//             "1" => Some(true),
//             "0" => Some(false),
//             _ => None,
//         }) {
//             None => return Err(DescriptorError { attribute : attributes::FS_BOOTLOADER_STATE, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_fw_version = match read_raw(path.join(attributes::FS_FW_VERSION)).and_then(| value | FwVersion::from_string(value).ok()) {
//             None => return Err(DescriptorError { attribute : attributes::FS_FW_VERSION, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_hw_version = match read_raw(path.join(attributes::FS_HW_VERSION)).and_then(| value | SimpleVersion::from_string(value).ok()) {
//             None => return Err(DescriptorError { attribute : attributes::FS_HW_VERSION, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_max_frame_size = match read_raw(path.join(attributes::FS_MAX_FRAME_SIZE)).and_then(| value | value.parse::<u16>().ok()) {
//             None => return Err(DescriptorError { attribute : attributes::FS_MAX_FRAME_SIZE, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_max_power_12v = match read_raw(path.join(attributes::FS_MAX_POWER_12V)).and_then(| value | value.parse::<u16>().ok()) {
//             None => return Err(DescriptorError { attribute : attributes::FS_MAX_POWER_12V, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_max_power_5v0 = match read_raw(path.join(attributes::FS_MAX_POWER_5V0)).and_then(| value | value.parse::<u16>().ok()) {
//             None => return Err(DescriptorError { attribute : attributes::FS_MAX_POWER_5V0, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_max_power_3v3 = match read_raw(path.join(attributes::FS_MAX_POWER_3V3)).and_then(| value | value.parse::<u16>().ok()) {
//             None => return Err(DescriptorError { attribute : attributes::FS_MAX_POWER_3V3, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_max_sclk_speed = match read_raw(path.join(attributes::FS_MAX_SCLK_SPEED)).and_then(| value | value.parse::<u32>().ok()) {
//             None => return Err(DescriptorError { attribute : attributes::FS_MAX_SCLK_SPEED, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_product_name = match read_raw(path.join(attributes::FS_PRODUCT_NAME)) {
//             None => return Err(DescriptorError { attribute : attributes::FS_MAX_SCLK_SPEED, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_protocol_version = match read_raw(path.join(attributes::FS_PROTOCOL_VERSION)).and_then(|value| SimpleVersion::from_string(value).ok())  {
//             None => return Err(DescriptorError { attribute : attributes::FS_PROTOCOL_VERSION, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_vendor_name = match read_raw(path.join(attributes::FS_VENDO_NAME)) {
//             None => return Err(DescriptorError { attribute : attributes::FS_MAX_SCLK_SPEED, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let r_serial_code = match read_raw(path.join(attributes::FS_SERIAL_CODE)) {
//             None => return Err(DescriptorError { attribute : attributes::FS_MAX_SCLK_SPEED, kind : DescriptorErrorKind::ParsingError}),
//             Some(_val) => _val,
//         };
//
//         let dev_file= PathBuf::from("/dev").join(slot_nr);
//         match dev_file.exists() {
//             true => (),
//             false => return Err(DescriptorError { attribute : attributes::FS_MAX_SCLK_SPEED, kind : DescriptorErrorKind::ParsingError}),
//         }
//
//
//
//         self.path = String::from(dev_file.to_str().unwrap());
//
//
//         self.bootloader_state = r_bootloader_state;
//         self.fw_version = r_fw_version;
//         self.hw_version = r_hw_version;
//         self.max_frame_size = r_max_frame_size;
//         self.max_power_12v = r_max_power_12v;
//         self.max_power_5v0 = r_max_power_5v0;
//         self.max_power_3v3 = r_max_power_3v3;
//         self.max_sclk_speed = r_max_sclk_speed;
//         self.product_name = r_product_name;
//         self.protocol_version = r_protocol_version;
//         self.serial_code = r_serial_code;
//         self.vendor_name = r_vendor_name;
//         self.vendor_product_id = r_vendor_product_id;
//
//
//         let regex = Regex::new("slot([0-9]*)").unwrap();
//
//         let mut cap = regex.captures_iter(self.path.as_str());
//         let test = cap.next().unwrap();
//         self.dev_adr = test[1].parse::<u16>().unwrap();
//         Ok(())
//     }
//
//     pub fn from_sysfs(parent_dir: &Path, slot_nr : &String) -> Result<Descriptor,DescriptorError> {
//         let mut result = Descriptor::default();
//
//         match result.update(parent_dir, slot_nr) {
//             Err(err) => return Err(err),
//             Ok(()) => return Ok(result),
//         };
//     }
//
//     fn update_from_sysfs(self, path: &PathBuf) -> Result<(), DescriptorError> {
//         unimplemented!()
//     }
// }

// #[cfg(test)]
// pub mod tests {
//
//     use super::Descriptor;
//     use super::attributes;
//     use std::fs;
//     use std::path;
//     use crate::sdbp::descriptor::attributes::FS_VENDOR_PRODUCT_ID;
//     use std::io::Write;
//     use std::path::{PathBuf, Path};
//
//     use super::super::super::generic::fw_version::FwVersion;
//     use super::super::super::generic::simple_version::SimpleVersion;
//
//     pub const TEST_SLOT_NAME: &str =  "slot0";
//
//     /**
//      Values for Test Descriptor
//     */
//     const TVAL_VENDOR_PRODUCT_ID: &str = "modules.noreya.tech/ion";
//     const TVAL_BOOTLOADER_STATE: &str = "0";
//     const TVAL_FW_VERSION: &str = "A.000.001.000";
//     const TVAL_HW_VERSION: &str = "001.000.000";
//     const TVAL_MAX_FRAME_SIZE: &str = "320";
//     const TVAL_MAX_POWER_12V: &str = "0";
//     const TVAL_MAX_POWER_5V0: &str = "0";
//     const TVAL_MAX_POWER_3V3: &str = "300";
//     const TVAL_MAX_SCLK_SPEED : &str = "8000";
//     const TVAL_PRODUCT_NAME: &str = "ION Module";
//     const TVAL_PROTOCOL_VERSION : &str = "001.000.000";
//     const TVAL_SERIAL_CODE: &str = "0000-0000-2033-3637-524B-430E-001F-0012";
//     const TVAL_VENDOR_NAME: &str = "NOREYA GmbH";
//
//     pub fn generate_descriptor()  {
//
//         fs::create_dir(TEST_SLOT_NAME).unwrap();
//
//         let module_path = path::Path::new(TEST_SLOT_NAME);
//
//         let mut fh = fs::File::create(module_path.join(FS_VENDOR_PRODUCT_ID)).unwrap();
//         fh.write(TVAL_VENDOR_PRODUCT_ID.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//         fh = fs::File::create(module_path.join(attributes::FS_BOOTLOADER_STATE)).unwrap();
//         fh.write(TVAL_BOOTLOADER_STATE.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//         fh = fs::File::create(module_path.join(attributes::FS_FW_VERSION)).unwrap();
//         fh.write(TVAL_FW_VERSION.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//         fh = fs::File::create(module_path.join(attributes::FS_HW_VERSION)).unwrap();
//         fh.write(TVAL_HW_VERSION.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//         fh = fs::File::create(module_path.join(attributes::FS_MAX_POWER_12V)).unwrap();
//         fh.write(TVAL_MAX_POWER_12V.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//
//         fh = fs::File::create(module_path.join(attributes::FS_MAX_POWER_5V0)).unwrap();
//         fh.write(TVAL_MAX_POWER_5V0.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//
//         fh = fs::File::create(module_path.join(attributes::FS_MAX_POWER_3V3)).unwrap();
//         fh.write(TVAL_MAX_POWER_3V3.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//
//         fh = fs::File::create(module_path.join(attributes::FS_MAX_FRAME_SIZE)).unwrap();
//         fh.write(TVAL_MAX_FRAME_SIZE.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//         fh = fs::File::create(module_path.join(attributes::FS_MAX_SCLK_SPEED)).unwrap();
//         fh.write(TVAL_MAX_SCLK_SPEED.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//
//         fh = fs::File::create(module_path.join(attributes::FS_PRODUCT_NAME)).unwrap();
//         fh.write(TVAL_PRODUCT_NAME.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//
//         fh = fs::File::create(module_path.join(attributes::FS_PROTOCOL_VERSION)).unwrap();
//         fh.write(TVAL_PROTOCOL_VERSION.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//
//         fh = fs::File::create(module_path.join(attributes::FS_SERIAL_CODE)).unwrap();
//         fh.write(TVAL_SERIAL_CODE.as_bytes()).unwrap();
//         fh.flush().unwrap();
//
//         fh = fs::File::create(module_path.join(attributes::FS_VENDO_NAME)).unwrap();
//         fh.write(TVAL_VENDOR_NAME.as_bytes()).unwrap();
//         fh.flush().unwrap();
//     }
//
//     pub fn clean_up () {
//         fs::remove_dir_all(TEST_SLOT_NAME).unwrap();
//     }
//
//     #[test]
//     fn test_descriptor() {
//
//         clean_up();
//         generate_descriptor();
//
//         let desc = Descriptor::from_sysfs(&PathBuf::from(Path::new(".")),&TEST_SLOT_NAME.to_string()).unwrap();
//
//         let expected_descriptor = Descriptor {
//             path: String::from(PathBuf::from("/dev").join(TEST_SLOT_NAME).to_str().unwrap()),
//             bootloader_state : match TVAL_BOOTLOADER_STATE {
//                 "1" => true,
//                 "0" => false,
//                 _ => false,
//             },
//             vendor_name : TVAL_VENDOR_NAME.to_string(),
//             vendor_product_id : TVAL_VENDOR_PRODUCT_ID.to_string(),
//             product_name : TVAL_PRODUCT_NAME.to_string(),
//             protocol_version: SimpleVersion::from_str(TVAL_PROTOCOL_VERSION).unwrap(),
//             fw_version : FwVersion::from_str(TVAL_FW_VERSION).unwrap(),
//             hw_version : SimpleVersion::from_str(TVAL_HW_VERSION).unwrap(),
//             max_frame_size : TVAL_MAX_FRAME_SIZE.parse().unwrap(),
//             max_sclk_speed :TVAL_MAX_SCLK_SPEED.parse().unwrap(),
//             max_power_12v : TVAL_MAX_POWER_12V.parse().unwrap(),
//             max_power_5v0 : TVAL_MAX_POWER_5V0.parse().unwrap(),
//             max_power_3v3 : TVAL_MAX_POWER_3V3.parse().unwrap(),
//             serial_code: TVAL_SERIAL_CODE.parse().unwrap()
//         };
//         assert_eq!(desc,expected_descriptor);
//     }
// }