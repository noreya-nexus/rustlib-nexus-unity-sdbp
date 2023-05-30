use std::io::{Error, ErrorKind};
use std::ffi::OsStr;

use crate::sdbp::request::custom::bmc::protocol::*;
use crate::sdbp::response::custom::bmc::check_status;
use crate::sdbp::response::SdbpResponse;
use std::path::{Path};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResetSuccess {
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetHubSuccess {
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetPortSuccess {
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct UsbHub {
    pub state: u8,
}

#[derive(Debug, Clone)]
pub struct UsbHubPort {
    pub slot0: u8,
    pub slot1: u8,
    pub slot2: u8,
    pub slot3: u8,
    pub slot4: u8,
    pub slot5: u8,
    pub slot6: u8,
    pub slot7: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsbHubRest {
    pub hub_state: String,
    pub port_slot_0: String,
    pub port_slot_1: String,
    pub port_slot_2: String,
    pub port_slot_3: String,
    pub port_slot_4: String,
    pub port_slot_5: String,
    pub port_slot_6: String,
    pub port_slot_7: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsbHubMapping {
    pub slot_number: u8,
    pub is_disabled: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsbPortSlotMapping {
    pub slot_number: u8,
    pub port_number: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct UsbDevice {
    pub slot: u8,
    pub usb_port: u8,
    pub manufacturer: String,
    pub product: String,
    pub serial_code: String,
    pub system_path: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsbHubDevices {
    pub devices: Vec<UsbDevice>,
}

pub fn map_state(state: u8) -> Result<String,String> {
    if state == 1 {
        return Ok("enabled".to_string());
    }
    else if state == 2 {
        return Ok("disabled".to_string());
    }
    else if state == 3 {
        return Ok("not available".to_string());
    } else {
        warn!("Could not map usb state");
        return Err("not mapped".to_string());
    }
}

impl SdbpResponse for UsbHubPort {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 11 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::usbhub::ID ||
            value[2] != classes::usbhub::operation_code::GET_USB_SLOT_STATE {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let slot0 = &value[3];
        let slot1 = &value[4];
        let slot2 = &value[5];
        let slot3 = &value[6];
        let slot4 = &value[7];
        let slot5 = &value[8];
        let slot6 = &value[9];
        let slot7 = &value[10];
        let check: [u8; 8] = [*slot0, *slot1, *slot2, *slot3, *slot4, *slot5, *slot6, *slot7];

        for element in &check {
            if *element == 0 || *element > 3 {
                return Err(Error::new(ErrorKind::InvalidData, "invalid port state"));
            }
        }

        Ok(UsbHubPort {
            slot0: slot0.clone(),
            slot1: slot1.clone(),
            slot2: slot2.clone(),
            slot3: slot3.clone(),
            slot4: slot4.clone(),
            slot5: slot5.clone(),
            slot6: slot6.clone(),
            slot7: slot7.clone(),
        })
    }
}

impl SdbpResponse for UsbHub {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::usbhub::ID ||
            value[2] != classes::usbhub::operation_code::GET_HUB_STATE {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let hub_state = &value[3];

        if *hub_state == 0 || *hub_state > 3 {
            return Err(Error::new(ErrorKind::InvalidData, "invalid hub state"));
        }

        Ok(UsbHub {
            state: hub_state.clone(),
        })
    }
}

impl SdbpResponse for SetHubSuccess {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::usbhub::ID ||
            value[2] != classes::usbhub::operation_code::SET_HUB_STATE {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status, "setting hub state: ".to_string(), "State is invalid".to_string(), true) {
            Ok(msg) => Ok(SetHubSuccess { status: msg }),
            Err(err) => Err(err),
        }
    }
}

impl SdbpResponse for SetPortSuccess {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::usbhub::ID ||
            value[2] != classes::usbhub::operation_code::SET_USB_SLOT_STATE {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status, "setting port state: ".to_string(), "State is invalid".to_string(), true) {
            Ok(msg) => Ok(SetPortSuccess { status: msg }),
            Err(err) => Err(err),
        }
    }
}

impl SdbpResponse for ResetSuccess {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 4 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::usbhub::ID ||
            value[2] != classes::usbhub::operation_code::HUB_RESET {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let status = &value[3];

        match check_status(status, "setting port state: ".to_string(), "State is invalid".to_string(), true) {
            Ok(msg) => Ok(ResetSuccess { status: msg }),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UsbHubPortMapping {
    pub slot0: UsbPortSlotMapping,
    pub slot1: UsbPortSlotMapping,
    pub slot2: UsbPortSlotMapping,
    pub slot3: UsbPortSlotMapping,
    pub slot4: UsbPortSlotMapping,
    pub slot5: UsbPortSlotMapping,
    pub slot6: UsbPortSlotMapping,
    pub slot7: UsbPortSlotMapping,
}


impl SdbpResponse for UsbHubPortMapping {
    fn from_raw(value: Vec<u8>) -> Result<Self, Error> {
        let value = value.as_slice();

        if value.len() != 11 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid length"));
        }

        if value[0] != CLASS_ID ||
            value[1] != classes::usbhub::ID ||
            value[2] != classes::usbhub::operation_code::GET_PORT_MAPPING {
            return Err(Error::new(ErrorKind::InvalidData, "Wrong Header"));
        }

        let slot0 = &value[3];
        let slot1 = &value[4];
        let slot2 = &value[5];
        let slot3 = &value[6];
        let slot4 = &value[7];
        let slot5 = &value[8];
        let slot6 = &value[9];
        let slot7 = &value[10];

        Ok(UsbHubPortMapping {
            slot0: UsbPortSlotMapping { slot_number: 0, port_number: *slot0 },
            slot1: UsbPortSlotMapping { slot_number: 1, port_number: *slot1 },
            slot2: UsbPortSlotMapping { slot_number: 2, port_number: *slot2 },
            slot3: UsbPortSlotMapping { slot_number: 3, port_number: *slot3 },
            slot4: UsbPortSlotMapping { slot_number: 4, port_number: *slot4 },
            slot5: UsbPortSlotMapping { slot_number: 5, port_number: *slot5 },
            slot6: UsbPortSlotMapping { slot_number: 6, port_number: *slot6 },
            slot7: UsbPortSlotMapping { slot_number: 7, port_number: *slot7 },
        })
    }
}

fn map_disabled_to_slot(disabled_slots: Vec<UsbHubMapping>, usb_slot_map: &Vec<UsbPortSlotMapping>) -> Vec<UsbPortSlotMapping> {
    let mut disabled_ports = Vec::new();
    for map in usb_slot_map {
        for disabled in &disabled_slots {
            if disabled.slot_number == map.slot_number {
                disabled_ports.push(UsbPortSlotMapping{ slot_number: map.slot_number, port_number: map.port_number })
            }
        }
    }
    return disabled_ports;
}

fn calculate_port_offset(disabled_slots: Vec<UsbPortSlotMapping>, usb_slot_map: &Vec<UsbPortSlotMapping>) -> Vec<UsbPortSlotMapping> {
    let mut enabled_slots = Vec::new();
    for map in usb_slot_map {
        const NO_USB_ON_SLOT: u8 = 0;
        let mut offset = 0;
        let mut found_disabled = false;
        for disabled in &disabled_slots {
            if map.port_number > disabled.port_number && disabled.port_number != NO_USB_ON_SLOT {
                offset += 1;
            }
            if map.slot_number == disabled.slot_number{
                found_disabled = true;
                break;
            }
        }
        if !found_disabled {
            enabled_slots.push(UsbPortSlotMapping{ slot_number: map.slot_number, port_number: map.port_number-offset });
        }
    }
    return enabled_slots;
}

pub fn get_usb_devices(disabled_slots: Vec<UsbHubMapping>, usb_slot_map: UsbHubPortMapping) -> Result<UsbHubDevices, String> {
    const USB_HUB: &str = "/dev/bus/usb/001/";
    let usb_slot_map= vec![usb_slot_map.slot0, usb_slot_map.slot1, usb_slot_map.slot2, usb_slot_map.slot3, usb_slot_map.slot4, usb_slot_map.slot5, usb_slot_map.slot6 , usb_slot_map.slot7 ];
    let disabled_slots = map_disabled_to_slot(disabled_slots, &usb_slot_map);
    let usb_ports_with_offset = calculate_port_offset(disabled_slots, &usb_slot_map);

    let mut enumerator = match udev::Enumerator::new() {
        Ok(val) => { val }
        Err(err) => { return Err(err.to_string()) }
    };
    match enumerator.match_subsystem("usb") {
        Ok(_) => {}
        Err(err) => { return Err(err.to_string()) }
    }
    let enumerator = match enumerator.scan_devices() {
        Ok(val) => { val }
        Err(err) => { return Err(err.to_string()) }
    };

    let mut usb_devices = Vec::new();
    for device in enumerator {
        if device.devtype().unwrap_or(OsStr::new("")).to_string_lossy() == "usb_device" {
            if device.devnode().unwrap_or(Path::new("/dev/")).to_string_lossy().starts_with(USB_HUB) {
                let mut manufacturer = device.attribute_value("manufacturer").unwrap_or(OsStr::new("")).to_string_lossy().to_string();
                let mut product = device.attribute_value("product").unwrap_or(OsStr::new("")).to_string_lossy().to_string();
                if product == "" {
                    product = device.property_value("ID_MODEL_FROM_DATABASE").unwrap_or(OsStr::new("Unknown")).to_string_lossy().to_string();
                }
                if manufacturer == "" {
                    manufacturer = device.property_value("ID_VENDOR_FROM_DATABASE").unwrap_or(OsStr::new("Unknown")).to_string_lossy().to_string();
                }
                let serial_code = device.attribute_value("serial").unwrap_or(OsStr::new("")).to_string_lossy().to_string();
                let sys_name = device.sysname().to_string_lossy().to_string();
                let sys_path = device.syspath().to_string_lossy().to_string();
                if sys_name.contains(".") {
                    let mut sys_name = sys_name.rsplitn(2, ".");
                    let port = sys_name.next();
                    match port {
                        None => {
                            warn!("Could not parse port number");
                            continue 
                        }
                        Some(port) => {
                            match port.parse::<u8>() {
                                Ok(port) => {
                                    for mapping in &usb_ports_with_offset {
                                        if mapping.port_number == port {
                                            let usb = UsbDevice {
                                                slot: mapping.slot_number as u8,
                                                usb_port: port as u8,
                                                manufacturer,
                                                product,
                                                serial_code,
                                                system_path: sys_path
                                            };
                                            usb_devices.push(usb);
                                            break;
                                        }
                                    }
                                }
                                Err(err) => {
                                    warn!("Could not parse port number: {}", err.to_string());
                                    continue
                                }
                            };
                        }
                    }
                }
            }
        }
    }
    usb_devices.sort();
    return Ok(UsbHubDevices{devices: usb_devices});
}
