use super::tlv;
use std::convert::TryFrom;

pub enum Error {
    UnknownCommand = 0xE000,
    DeviceNotConnected = 0xE001,
    InvalidLength = 0xE002,
    InvalidParameter = 0xE003,
    TlvError = 0xE004,
    VirtualDeviceError = 0xE005
}

impl Error {

    pub fn to_bytes(&self) -> [u8;2] {

        let result = match self {
            Error::UnknownCommand => { (Error::UnknownCommand as u16).to_ne_bytes() },
            Error::DeviceNotConnected => { (Error::DeviceNotConnected as u16).to_ne_bytes() },
            Error::InvalidParameter => { (Error::InvalidParameter as u16).to_ne_bytes() },
            Error::InvalidLength => { (Error::InvalidLength as u16).to_ne_bytes() },
            Error::TlvError => { (Error::TlvError as u16).to_ne_bytes() },
            Error::VirtualDeviceError => { (Error::VirtualDeviceError as u16).to_ne_bytes() },
        };
        result
    }
}

impl TryFrom<tlv::error::Error> for Error {
    type Error = ();

    fn try_from(_value: tlv::error::Error) -> Result<Self, Self::Error> {
      Ok(Error::TlvError)
    }
}