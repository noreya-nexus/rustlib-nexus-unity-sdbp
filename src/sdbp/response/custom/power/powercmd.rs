use std::io::{ErrorKind,Error};
use crate::sdbp::response::SdbpResponse;
use crate::sdbp::request::custom::power::protocol::*;


#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct Source {
    pub source_3v3 : u16, // IMPROVEMENT: Add unit suffix
    pub source_5v0: u16, // IMPROVEMENT: Add unit suffix
    pub source_12v : u16, // IMPROVEMENT: Add unit suffix
    pub source_total : u16, // IMPROVEMENT: Add unit suffix

    pub efficiency_factor_3v3 : u8,
    pub efficiency_factor_5v0: u8,
    pub efficiency_factor_12v : u8,
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct Limit {
    pub status : String,
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct VoltageStatus {
    pub voltage_3v3 : u16,
    pub voltage_5v0 : u16,
    pub voltage_12v : u16,

    pub current_3v3 : u32,
    pub current_5v0 : u32,
    pub current_12v : u32,

    pub limit_3v3: u32,
    pub limit_5v0: u32,
    pub limit_12v: u32,
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct ProtectionStatus{
    pub opp_protection_cnt : u16,
    pub ovp_cnt_3v3 : u16,
    pub uvp_cnt_3v3 : u16,

    pub ovp_cnt_5v0 : u16,
    pub uvp_cnt_5v0 : u16,

    pub ovp_cnt_12v : u16,
    pub uvp_cnt_12v : u16,

    pub ocp_cnt_3v3: u16,
    pub ocp_cnt_5v0: u16,
    pub ocp_cnt_12v: u16,

    pub pmc_temperature: u16, // IMPROVEMENT: Add unit suffix
    pub total_power_overload : u16,
}

impl SdbpResponse for ProtectionStatus {


    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {

        let value = value.as_slice();
        if value.len() != 27 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::power_class::ID ||
            value[2] != classes::power_class::operation_code::OPP_CNT_STATUS {
            return Err(Error::new(ErrorKind::InvalidData,"Wrong Header"))
        }

        Ok(ProtectionStatus{
            opp_protection_cnt: u16::from_be_bytes([value[3],value[4]]),
            ovp_cnt_3v3: u16::from_be_bytes([value[5],value[6]]),
            uvp_cnt_3v3: u16::from_be_bytes([value[7],value[8]]),
            ovp_cnt_5v0: u16::from_be_bytes([value[9],value[10]]),
            uvp_cnt_5v0: u16::from_be_bytes([value[11],value[12]]),
            ovp_cnt_12v: u16::from_be_bytes([value[13],value[14]]),
            uvp_cnt_12v: u16::from_be_bytes([value[15],value[16]]),
            ocp_cnt_3v3: u16::from_be_bytes([value[17],value[18]]),
            ocp_cnt_5v0: u16::from_be_bytes([value[19],value[20]]),
            ocp_cnt_12v: u16::from_be_bytes([value[21],value[22]]),
            pmc_temperature: u16::from_be_bytes([value[23],value[24]]),
            total_power_overload: u16::from_be_bytes([value[25],value[26]])
        })

    }
}

impl SdbpResponse for VoltageStatus {

    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {

        let value = value.as_slice();
        if value.len() != 33 {
            return Err(Error::new(ErrorKind::InvalidData,format!("Invalid length {}",value.len())));
        }

        if value[0] != 3 || value[1] != 1 || value[2] != 4 {
            return Err(Error::new(ErrorKind::InvalidData,"Wrong Header"))
        }

        Ok(VoltageStatus{
            voltage_3v3: u16::from_be_bytes([value[3],value[4]]),
            voltage_5v0: u16::from_be_bytes([value[5],value[6]]),
            voltage_12v: u16::from_be_bytes([value[7],value[8]]),
            current_3v3: u32::from_be_bytes([value[9],value[10],value[11],value[12]]),
            current_5v0: u32::from_be_bytes([value[13],value[14],value[15],value[16]]),
            current_12v: u32::from_be_bytes([value[17],value[18],value[19],value[20]]),
            limit_3v3: u32::from_be_bytes([value[21],value[22],value[23],value[24]]),
            limit_5v0: u32::from_be_bytes([value[25],value[26],value[27],value[28]]),
            limit_12v: u32::from_be_bytes([value[29],value[30],value[31],value[32]])
        })
    }
}

impl SdbpResponse for Limit {

    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {

        let value = value.as_slice();
        if value.len() != 6 {
            return Err(Error::new(ErrorKind::InvalidData,"Invalid length"));
        }

        if value[0] != 3 || value[1] != 1 || value[2] != 3 {
            return Err(Error::new(ErrorKind::InvalidData,"Wrong Header"))
        }

        let mut status = "error";
        if value[3] == 1 && value[4] == 1 && value[5] == 1 {
            status = "success";
        }

        Ok(Limit{
            status: status.to_string()
        })
    }
}

impl SdbpResponse for Source {

    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {

        let value = value.as_slice();

        if value.len() != 14 {
            return Err(Error::new(ErrorKind::InvalidData,"Invalid length"));
        }

        if  value[0] != CLASS_ID ||
            value[1] != classes::power_class::ID ||
            value[2] != classes::power_class::operation_code::SOURCE {
            return Err(Error::new(ErrorKind::InvalidData,"Wrong Header"))
        }

        let source_3v3 = &value[3..5];
        let source_5v0 = &value[5..7];
        let source_12v = &value[7..9];
        let source_total = &value[9..11];
        let eff_3v3 = value[11];
        let eff_5v0 = value[12];
        let eff_12v = value[13];

        let source_3v3 = u16::from_ne_bytes([source_3v3[1], source_3v3[0]]);
        let source_5v0 = u16::from_ne_bytes([source_5v0[1], source_5v0[0]]);
        let source_12v = u16::from_ne_bytes([source_12v[1], source_12v[0]]);
        let source_total = u16::from_ne_bytes([source_total[1], source_total[0]]);

        Ok(Source {
            source_3v3: source_3v3,
            source_5v0: source_5v0,
            source_12v: source_12v,
            source_total: source_total,
            efficiency_factor_3v3: eff_3v3,
            efficiency_factor_5v0: eff_5v0,
            efficiency_factor_12v: eff_12v,
        })
    }
}