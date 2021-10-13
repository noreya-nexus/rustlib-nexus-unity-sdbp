use std::convert::TryFrom;

use super::*;
use super::error::Error;
use crate::datatypes::{Version, AdvancedVersion,BootloaderState};

pub struct Parser {}

impl Parser {


    pub fn parse_bootloader_state(value: &[u8]) -> Result<TlvValue, Error> {
        if value.len() != 1 {
            trace!("Parsing bool failed");
            return Err(Error::ParsingError);
        }

      match BootloaderState::try_from(value[0]){
            Ok(value) => Ok(TlvValue::from(value)),
            Err(_err) =>  Err(Error::ParsingError),
        }
    }
    pub fn parse_bool(value: &[u8]) -> Result<TlvValue, Error> {
        if value.len() != 1 {
            trace!("Parsing bool failed");
            return Err(Error::ParsingError);
        }

        match value[0] {
            0 => Ok(TlvValue::from(false)),
            1 => Ok(TlvValue::from(true)),
            _ => {
                trace!("Parsing bool failed");
                Err(Error::ParsingError)
            },
        }
    }

    pub fn parse_string(value: &[u8]) -> Result<TlvValue, Error> {
        if value.len() < 2 {
            trace!("Parsing string failed");
            return Err(Error::ParsingError);
        }

        match String::from_utf8(value.to_vec()) {
            Ok(value) => Ok(TlvValue::String(value)),
            Err(_err) =>{
                trace!("Parsing from utf8 failed");
                Err(Error::ParsingError)
            }
        }
    }

    pub fn parse_advanced_version(value: &[u8]) -> Result<TlvValue, Error> {

        if value.len() != 7 {
            return Err(Error::ParsingError);
        }

        match AdvancedVersion::try_from(&value[..]) {
            Err(_err) => {
                trace!("Parsing version failed");
                Err(Error::ParsingError)
            },
            Ok(version) => {
                Ok(TlvValue::from(version))
            },
        }
    }

    pub fn parse_version(value: &[u8]) -> Result<TlvValue, Error> {

        if value.len() != 6 {
            trace!("Parsing version failed");
            return Err(Error::ParsingError);
        }

        match Version::try_from(&value[..]) {
            Err(_err) => {
                trace!("Parsing version failed");
                Err(Error::ParsingError)
            },
            Ok(version) => {
                Ok(TlvValue::from(version))
            },
        }
    }

    pub fn parse_u8(value: &[u8]) -> Result<TlvValue, Error> {
        if value.len() != 1 {
            trace!("Parsing u8 failed");
            return Err(Error::ParsingError);
        }
        Ok(TlvValue::U8(u8::from_ne_bytes([value[0]])))
    }

    pub fn parse_u16(value: &[u8]) -> Result<TlvValue, Error> {
        if value.len() != 2 {
            trace!("Parsing u16 failed");
            return Err(Error::ParsingError);
        }

        Ok(TlvValue::U16(u16::from_ne_bytes([value[0], value[1]])))
    }

    pub fn parse_u32(value: &[u8]) -> Result<TlvValue, Error> {
        if value.len() != 4 {
            trace!("Parsing u32 failed");
            return Err(Error::ParsingError);
        }

        Ok(TlvValue::U32(u32::from_ne_bytes([value[0], value[1], value[2], value[3]])))
    }
}