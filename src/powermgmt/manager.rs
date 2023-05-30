use std::time::Duration;
use std::io::Error;
use crate::util::{UnixStreamReader, Connection};
use std::os::unix::net::UnixStream;


#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct PowerBudgetRequest {
    pub protocol_version : String,
    pub msg_type : String,
    pub slot_number : u8,
    pub max_power_3v3 : u16,
    pub max_power_5v0 : u16,
    pub max_power_12v : u16,
}

impl PowerBudgetRequest {
    fn new(slot: u8, max_power_3v3 : u16, max_power_5v0: u16, max_power_12v: u16) -> PowerBudgetRequest {
        PowerBudgetRequest {
            protocol_version : "1.0.0".to_string(),
            msg_type : "request".to_string(),
            slot_number: slot,
            max_power_3v3,
            max_power_5v0,
            max_power_12v
        }
    }
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct PowerBudgetResponse {
    pub protocol_version : String,
    pub msg_type : String,
    pub successful : bool,
    pub to_much_power_3v3 : u16,
    pub to_much_power_5v0 : u16,
    pub to_much_power_12v : u16,
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct PowerFinishRequest {
    pub protocol_version : String,
    pub msg_type : String,
    pub action_done : bool,
}

impl PowerFinishRequest {
    fn new() -> PowerFinishRequest {
        PowerFinishRequest {
            protocol_version : "1.0.0".to_string(),
            msg_type : "request".to_string(),
            action_done : true,
        }
    }
}

#[derive(Debug,Clone, serde::Serialize, serde::Deserialize)]
pub struct PowerFinishResponse {
    pub protocol_version : String,
    pub msg_type : String,
    pub successful : bool,
    pub message : String,
}

pub struct PowerManager {
    com : UnixStreamReader,
}


impl PowerManager {


    pub fn new(socket_path : String, timeout : Option<Duration>) -> Result<PowerManager,Error> {

        let stream = match UnixStream::connect(socket_path) {
            Ok(value) => value,
            Err(err) =>{
                trace!("{}",err);
                return Err(err);
            }
        };
        Ok(PowerManager{com : UnixStreamReader::from_unix_stream(stream,timeout)})
            }

    pub fn request(&mut self,slot : u8, max_power_3v3 : u16, max_power_5v: u16, max_power_12v : u16) -> Result <PowerBudgetResponse,String>{

        let tmp = PowerBudgetRequest::new(slot,max_power_3v3,max_power_5v,max_power_12v);
        let string = serde_json::to_string(&tmp).expect("Could not convert struct to string");

        debug!("Request: {:?}",string);

        match self.com.write_msg(string.as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                return Err("Power management request failed".to_string());
            }
        };
        let response =  match self.com.read_msg() {
            Ok(val) => {val}
            Err(_) => {
                return Err("Power management request failed (response)".to_string());
            }
        };

        debug!("Response: {:?}",String::from_utf8(response.clone()));

        let json : PowerBudgetResponse = serde_json::from_slice(response.as_slice()).expect("Could not convert json to struct");

        return Ok(json);
    }


    pub fn finish_request(&mut self)  -> Result<PowerFinishResponse,String> {
        let tmp = PowerFinishRequest::new();
        let string = serde_json::to_string(&tmp).expect("Could not convert struct to string");

        match self.com.write_msg(string.as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                return Err("Power management finish request failed".to_string());
            }
        };
        let response = match self.com.read_msg() {
            Ok(val) => {val}
            Err(_) => {
                return Err("Power management finish request failed (response)".to_string());
            }
        };
        debug!("Finish response: {:?}",String::from_utf8(response.clone()));
        let json : PowerFinishResponse = serde_json::from_slice(response.as_slice()).expect("Could not convert json to struct");

        return Ok(json);
    }
}