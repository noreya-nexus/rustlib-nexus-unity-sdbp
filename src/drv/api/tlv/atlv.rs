use std::cmp::Ordering;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::ops::{Index, IndexMut};
use std::slice::{Iter,IterMut};

use super::error::*;
use super::parser::Parser;
use crate::datatypes::{AdvancedVersion, Version, BootloaderState};

pub trait IntoBytes {
    fn into_bytes(&self) -> Vec<u8>;
}



#[derive(Debug,Clone,Eq,PartialEq,PartialOrd)]
pub enum Tag {
    InfoBlock =  0x1000,
    DriverVersion =  0x1001,
    SdbpkDriverVersion =  0x1002,
    DeviceSession =  0x1003,
    DeviceBlock =  0x2000,
    DeviceAddress =  0x2001,
    ProductName =  0x2002,
    VendorName =  0x2003,
    VendorProductId =  0x2004,
    BootloaderState =  0x2005,
    FirmwareVersion =  0x2006,
    HardwareVersion =  0x2007,
    SupportedSdbpVersion =  0x2008,
    MaxFrameSize =  0x2009,
    MaxSclkSpeed =  0x200A,
    MaxPower12v =  0x200B,
    MaxPower5v =  0x200C,
    MaxPower3v3 =  0x200D,
    SerialNumber =  0x200E,
    DeviceTunnel =  0x3000,
    Response =  0x3001,
    ErrorValue = 0xEEEE,
    ErrorMsg = 0xEEEF,
}

impl std::cmp::Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[allow(dead_code)]
impl Tag {

    fn value(&self) -> u16 {

        let value = match self {
            Tag::InfoBlock => 0x1000,
            Tag::DriverVersion => 0x1001,
            Tag::SdbpkDriverVersion => 0x1002,
            Tag::DeviceSession => 0x1003,
            Tag::DeviceBlock => 0x2000,
            Tag::DeviceAddress => 0x2001,
            Tag::ProductName => 0x2002,
            Tag::VendorName => 0x2003,
            Tag::VendorProductId => 0x02004,
            Tag::BootloaderState => 0x2005,
            Tag::FirmwareVersion => 0x2006,
            Tag::HardwareVersion => 0x2007,
            Tag::SupportedSdbpVersion => 0x2008,
            Tag::MaxFrameSize => 0x2009,
            Tag::MaxSclkSpeed => 0x200A,
            Tag::MaxPower12v => 0x200B,
            Tag::MaxPower5v => 0x200C,
            Tag::MaxPower3v3 => 0x200D,
            Tag::SerialNumber => 0x200E,
            Tag::DeviceTunnel => 0x3000,
            Tag::Response => 0x3001,
            Tag::ErrorValue => 0xEEEE,
            Tag::ErrorMsg => 0xEEEF,
        } as u16;
        value
    }

    fn to_value(value : Tag) -> u16 {
        value.value()
    }
}


impl TryFrom<u16> for Tag {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let result  = match value {
            x if  x == ( Tag::InfoBlock as u16 ) => Ok(Tag::InfoBlock ),
            x if  x == ( Tag::DriverVersion as u16 ) => Ok(Tag::DriverVersion ),
            x if  x == ( Tag::SdbpkDriverVersion as u16 ) => Ok(Tag::SdbpkDriverVersion ),
            x if  x == ( Tag::DeviceSession as u16 ) => Ok(Tag::DeviceSession ),
            x if  x == ( Tag::DeviceBlock as u16 ) => Ok(Tag::DeviceBlock),
            x if  x == ( Tag::DeviceAddress as u16 ) => Ok(Tag::DeviceAddress),
            x if  x == ( Tag::ProductName as u16 ) => Ok(Tag::ProductName),
            x if  x == ( Tag::VendorName as u16 ) => Ok(Tag::VendorName),
            x if  x == ( Tag::VendorProductId as u16 ) => Ok(Tag::VendorProductId),
            x if  x == ( Tag::BootloaderState as u16 ) => Ok(Tag::BootloaderState),
            x if  x == ( Tag::FirmwareVersion as u16 ) => Ok(Tag::FirmwareVersion),
            x if  x == ( Tag::HardwareVersion as u16 ) => Ok(Tag::HardwareVersion),
            x if  x == ( Tag::SupportedSdbpVersion as u16 ) => Ok(Tag::SupportedSdbpVersion),
            x if  x == ( Tag::MaxFrameSize as u16 ) => Ok(Tag::MaxFrameSize),
            x if  x == ( Tag::MaxSclkSpeed as u16 ) => Ok(Tag::MaxSclkSpeed),
            x if  x == ( Tag::MaxPower12v as u16 ) => Ok(Tag::MaxPower12v),
            x if  x == ( Tag::MaxPower5v as u16 ) => Ok(Tag::MaxPower5v),
            x if  x == ( Tag::MaxPower3v3 as u16 ) => Ok(Tag::MaxPower3v3),
            x if  x == ( Tag::SerialNumber as u16 ) => Ok(Tag::SerialNumber),
            x if  x == ( Tag::DeviceTunnel as u16 ) => Ok(Tag::DeviceTunnel),
            x if  x == ( Tag::Response as u16 ) => Ok(Tag::Response),
            x if  x == ( Tag::ErrorValue as u16 ) => Ok(Tag::ErrorValue),
            x if  x == ( Tag::ErrorMsg as u16 ) => Ok(Tag::ErrorMsg),
            _ => Err(()),
        };
        result
    }
}

impl IntoBytes for Tag {
    fn into_bytes(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        result.extend_from_slice(&self.value().to_ne_bytes());
        result
    }
}
#[derive(Debug)]
pub enum TlvValue {

    Version(Version),
    AdvancedVersion(AdvancedVersion),
    Bytes(Vec<u8>),
    String(String),
    U8(u8),
    U16(u16),
    U32(u32),
    Bool(bool),
    Array(Vec<(Tag,TlvValue)>),
    Empty,
}

static NULL: TlvValue = TlvValue::Empty;

impl TlvValue {

    pub fn new() -> TlvValue {
        TlvValue::Array(Vec::new())
    }

    pub fn new_array() -> TlvValue {
        TlvValue::Array(Vec::new())
    }

    pub fn get_mut(&mut self, tag : &Tag) -> Option<&mut TlvValue> {
        let result = match self {
            TlvValue::Array(tree) => {
                let mut result = None;
                for (leaf_tag, leaf_value) in tree.iter_mut() {
                    if *leaf_tag == *tag {
                        result = Some(leaf_value);
                        break;
                    }
                }
                result
            },
            _ => None,
        };
        result
    }

    pub fn get(&self, tag : &Tag) -> Option<&TlvValue> {
        let result = match self {
            TlvValue::Array(tree) => {
                let mut result = None;
                for (leaf_tag, leaf_value) in tree {
                    if *leaf_tag == *tag {
                        result = Some(leaf_value);
                        break;
                    }
                }
                result
            },
            _ => None,
        };
        result
    }

    pub fn push(&mut self, tag: Tag, value : TlvValue) -> Option<&mut TlvValue> {

        match self {

            TlvValue::Array(tree) => {
                tree.push((tag.clone(),value));
                Some(&mut tree.last_mut().unwrap().1)
            },
            _ => None,
        }
    }

    pub fn as_version(&self) -> Option<&Version> {
        match self {

            TlvValue::Version(version) => Some(version),
            _ => None
        }
    }

    pub fn as_advanced_version(&self) -> Option<&AdvancedVersion> {
        match self {
            TlvValue::AdvancedVersion(version) => Some(version),
            _ => None
        }

    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TlvValue::Bool(version) => Some(*version),
            _ => None
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            TlvValue::String(str) => Some(str),
            _ => None
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        match self {
            TlvValue::U8(nr) => Some(*nr),
            _ => None
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match self {
            TlvValue::U16(nr) => Some(*nr),
            _ => None
        }
    }

    pub fn as_32(&self) -> Option<u32> {
        match self {
            TlvValue::U32(nr) => Some(*nr),
            _ => None
        }
    }

    pub fn as_bytes(&self) -> Option<&Vec<u8>> {
        match self {
            TlvValue::Bytes(vec) => Some(vec),
            _ => None
        }
    }
    pub fn members_mut<'a>(&'a mut self) -> IterMut<'a,(Tag,TlvValue)> {
        match self {

            TlvValue::Array(vec) => vec.iter_mut(),
            _ => [].iter_mut()
        }
    }

    pub fn members<'a>(&'a self) -> Iter<'a,(Tag,TlvValue)> {
        match self {

            TlvValue::Array(vec) => vec.iter(),
            _ => [].iter()
        }
    }
}


#[allow(unreachable_patterns)]
impl TryFrom<&[u8]> for TlvValue {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut tree = Vec::<(Tag,TlvValue)>::new();


        let mut offset = 0;

        while offset < value.len()
        {
            if value.len() - offset < 4 {
                trace!("Parsing initial offset failed");
                return Err(Error::ParsingError)
            }

            let raw_tag = u16::from_ne_bytes([value[offset], value[offset+1]]);
            let raw_len = u16::from_ne_bytes([value[offset+2], value[offset+3]]) as usize;
            offset += 4;

            if offset > value.len() {
                trace!("Parsing offset failed");
                return Err(Error::ParsingError)
            }

            let tag = match Tag::try_from(raw_tag) {

                Ok(value) => value,
                Err(_err) => return Err(Error::InvalidTag),
            };

            let offset_end = offset+raw_len;
            let parsed_value = match tag {
                Tag::InfoBlock => TlvValue::try_from(&value[offset..offset+raw_len]),
                Tag::DriverVersion => Parser::parse_version(&value[offset..offset+raw_len]),
                Tag::DeviceSession => Parser::parse_string(&value[offset..offset+raw_len]),
                Tag::SdbpkDriverVersion => Parser::parse_version(&value[offset..offset+raw_len]),
                Tag::DeviceBlock => TlvValue::try_from(&value[offset..offset+raw_len]),
                Tag::DeviceAddress => Parser::parse_u16(&value[offset..offset_end]),
                Tag::ProductName => Parser::parse_string(&value[offset..offset_end]),
                Tag::VendorName => Parser::parse_string(&value[offset..offset_end]),
                Tag::VendorProductId => Parser::parse_string(&value[offset..offset_end]),
                Tag::BootloaderState => Parser::parse_string(&value[offset..offset_end]),
                Tag::FirmwareVersion => Parser::parse_advanced_version(&value[offset..offset_end]),
                Tag::HardwareVersion => Parser::parse_version(&value[offset..offset_end]),
                Tag::SupportedSdbpVersion => Parser::parse_version(&value[offset..offset_end]),
                Tag::MaxFrameSize => Parser::parse_u16(&value[offset..offset_end]),
                Tag::MaxSclkSpeed => Parser::parse_u32(&value[offset..offset_end]),
                Tag::MaxPower12v => Parser::parse_u16(&value[offset..offset_end]),
                Tag::MaxPower5v => Parser::parse_u16(&value[offset..offset_end]),
                Tag::MaxPower3v3 => Parser::parse_u16(&value[offset..offset_end]),
                Tag::SerialNumber => Parser::parse_string(&value[offset..offset_end]),
                Tag::DeviceTunnel => TlvValue::try_from(&value[offset..offset_end]),
                Tag::Response => Ok(TlvValue::Bytes(Vec::from(&value[offset..offset_end]))),
                Tag::ErrorValue => Parser::parse_u16(&value[offset..offset_end]),
                Tag::ErrorMsg  => Parser::parse_string(&value[offset..offset_end]),
                _ => {
                    trace!("Unknown tag failed {:?},",tag);
                    Err(Error::ParsingError)
                },
            };

            match parsed_value {
                Ok(value) => tree.push((tag,value)),
                Err(err)=> return Err(err),
            };

            offset += raw_len;
        }
        Ok(TlvValue::Array(tree))
    }
}

impl From<Version> for TlvValue {
    fn from( version: Version) -> Self {
        TlvValue::Version(version)
    }
}

impl From<BootloaderState> for TlvValue {
    fn from( state: BootloaderState) -> Self {

        let value = match state {
            BootloaderState::NotSupported => 0,
            BootloaderState::Supported => 1,
            BootloaderState::BootloaderMode => 2,
            BootloaderState::NotInitialized => 3,
        };

        TlvValue::U8(value)
    }
}

impl From<AdvancedVersion> for TlvValue {
    fn from( version: AdvancedVersion) -> Self {
        TlvValue::AdvancedVersion(version)
    }
}

impl From<u32> for TlvValue {
    fn from( value: u32) -> Self {
        TlvValue::U32(value)
    }
}

impl From<u16> for TlvValue {
    fn from(value: u16) -> Self {
       TlvValue::U16(value)
    }
}

impl From<u8> for TlvValue {
    fn from( value: u8) -> Self {
        TlvValue::U8(value)
    }
}

impl From<bool> for TlvValue {
    fn from(value: bool) -> Self {
        TlvValue::Bool(value)
    }
}

impl From<String> for TlvValue {
    fn from(value: String) -> Self {
        TlvValue::String(value)
    }
}

impl From<Vec<u8>> for TlvValue {
    fn from(value: Vec<u8>) -> Self {
        TlvValue::Bytes(value)
    }
}

impl From<()> for TlvValue {
    fn from(_: ()) -> Self {
       TlvValue::Empty
    }
}


impl TryInto<Version> for TlvValue {

    type Error = Error;

    fn try_into(self) -> Result<Version, Self::Error> {
        match self {
            TlvValue::Version(version) => Ok(version),
            _ => Err(Error::ParsingError),
        }
    }
}

impl TryInto<AdvancedVersion> for TlvValue {
    type Error = Error;


    fn try_into(self) -> Result<AdvancedVersion, Self::Error> {
        match self {
            TlvValue::AdvancedVersion(version) => Ok(version),
            _ => Err(Error::ParsingError),
        }
    }
}



impl IntoBytes for TlvValue {

    fn into_bytes(&self) -> Vec<u8> {

        let mut result = Vec::<u8>::new();

        match self  {
            TlvValue::Bytes(value ) => result.extend_from_slice(value.as_slice()),
            TlvValue::Bool(value) => result.push(*value as u8),
            TlvValue::String(value)=> result.extend_from_slice(value.as_bytes()),
            TlvValue::U8(value) => result.push(*value),
            TlvValue::U16(value) => result.extend_from_slice(&value.to_ne_bytes()),
            TlvValue::U32(value) => result.extend_from_slice(&value.to_ne_bytes()),

            TlvValue::AdvancedVersion(version) =>result.extend(version.to_bytes()),
            TlvValue::Version(version) => result.extend(version.to_bytes()),//version.into_bytes();

            TlvValue::Array(tree) => {

                for (tag, value) in tree {
                    let value_bytes = value.into_bytes();
                    result.extend(tag.into_bytes());
                    result.extend(&(value_bytes.len() as u16).to_ne_bytes());
                    result.extend(value_bytes);
                }
            },
            _ => (),
        };
        result
    }
}


impl Index<Tag> for TlvValue {

    type Output = TlvValue;

    fn index(&self, index: Tag) -> &TlvValue {

        match *self {
            TlvValue::Array(ref _vec) => self.get(&index).unwrap(),
            _ => &NULL,
        }
    }
}

 impl IndexMut<Tag> for TlvValue {
    fn index_mut(&mut self, index: Tag) -> &mut TlvValue {

        match self {

            TlvValue::Array(_) => {

                let in_bound = self.get(&index);
                if in_bound.is_some() {
                   self.get_mut(&index).unwrap()
                } else {
                    self.push(index,TlvValue::Empty).unwrap()
                }
            }
            _ => {
                *self = TlvValue::new_array();
                self.push(index, TlvValue::Empty).unwrap()
            }
        }
    }
}


#[cfg(test)]
mod tests {

/*    macro_rules! assert_result {
        ($result:expr,$msg:expr) => {
            match $result {
                Ok(value) => value,
                Err(_) => panic!($msg),
            }
        };
    }*/

    macro_rules! assert_option {
        ($result:expr,$msg:expr) => {
            match $result {
                Some(value) => value,
                None => panic!($msg),
            }
        };
    }


    use std::time::Instant;
    use super::super::*;
    use std::convert::TryFrom;
    use crate::datatypes::Version;


    #[test]
    fn tlv_create() {

        let now = Instant::now();

        let version = Version::new(1,1,2);

        let mut  tlv = TlvValue::new();
        let block = tlv.push(Tag::InfoBlock,TlvValue::new_array()).unwrap();

        block.push(Tag::DriverVersion, TlvValue::Version(version));

        block.push(Tag::ErrorMsg, TlvValue::from("This is a test.".to_string()));
        block.push(Tag::SerialNumber, TlvValue::from("This is a test.".to_string()));
        block.push(Tag::SerialNumber, TlvValue::from("This is a test.".to_string()));
        block.push(Tag::SerialNumber, TlvValue::from("This is a test.".to_string()));
        block.push(Tag::SerialNumber, TlvValue::from("This is a test.".to_string()));

        let block = assert_option!(tlv.push(Tag::InfoBlock,TlvValue::new_array()),"Failed to add Array");
        block.push(Tag::DriverVersion, TlvValue::Version(Version::from_str("00002.00002.00002").unwrap()));
        block.push(Tag::SerialNumber, TlvValue::from("This is no test".to_string()));
        let bytes = tlv.into_bytes();
        let _result = TlvValue::try_from(bytes.as_slice());
        println!("Test duration: {} us",now.elapsed().as_micros());
    }

    #[test]
    fn tlv_index() {


        let now = Instant::now();

        let mut tlv = TlvValue::new();
        tlv[Tag::DeviceTunnel] = TlvValue::new_array();
        tlv[Tag::DeviceTunnel][Tag::DriverVersion] = TlvValue::Version(Version::from_str("00002.00002.00002").unwrap());


        let bytes = tlv.into_bytes();
        let result = TlvValue::try_from(bytes.as_slice()).unwrap();
        println!("{:?}",result);
        println!("{}",result.get(&Tag::DeviceTunnel).unwrap().get(&Tag::DriverVersion).unwrap().as_version().unwrap());

        println!("Test duration: {} us",now.elapsed().as_micros());
    }
}






