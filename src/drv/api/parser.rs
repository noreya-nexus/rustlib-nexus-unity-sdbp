use super::*;
use crate::datatypes::{Descriptor, Version};

pub struct ModApi {}

impl ModApi {

    pub fn parse(input: &[u8]) -> (Option<Command>,Option<Request>) {

        let request = Request::from_bytes(input);

        let result = match &request {
            Some(value) => Command::from_frame(value),
            None => None,
        };
        (result,request)
    }

    pub fn info(version: &Version, sdbpk_version: &Version) -> Response {
        Vec::<u8>::new();


        let mut tlv = TlvValue::new();
        let array = tlv.push(Tag::InfoBlock,TlvValue::new_array()).unwrap();

        array.push(Tag::DriverVersion,TlvValue::from(version.clone()));
        array.push(Tag::SdbpkDriverVersion,TlvValue::from(sdbpk_version.clone()));

        let mut response = Response::new_empty_response();
        response.append_bytes(tlv.into_bytes().as_slice());
        response
    }


    fn get_devices(devices : &Vec<Descriptor>, mode: bool, dev_adr : Option<u16>) -> TlvValue{
        let mut tlv = TlvValue::new();

        for device in devices {

            if !dev_adr.is_none() && dev_adr.unwrap() != device.adr() {
                continue
            }

            trace!("Descriptor: {:?}",device);
            let array = tlv.push(Tag::DeviceBlock,TlvValue::new_array()).unwrap();

            if mode == false {

                array.push(Tag::DeviceAddress,TlvValue::from(device.adr()));
                array.push(Tag::ProductName,TlvValue::from(device.product_name().clone()));
                array.push(Tag::VendorName,TlvValue::from(device.vendor_name().clone()));
                array.push(Tag::VendorProductId,TlvValue::from(device.vendor_product_id().clone()));
                array.push(Tag::BootloaderState,TlvValue::from(device.bootloader_state()));

                array.push(Tag::FirmwareVersion,TlvValue::from(device.fw_version().clone()));
                array.push(Tag::HardwareVersion,TlvValue::from(device.hw_version().clone()));
                array.push(Tag::SupportedSdbpVersion,TlvValue::from(device.protocol_version().clone()));

                array.push(Tag::MaxFrameSize,TlvValue::from(device.max_frame_size()));
                array.push(Tag::MaxSclkSpeed,TlvValue::from(device.max_sclk_speed()));
                array.push(Tag::MaxPower12v,TlvValue::from(device.max_power_12v()));
                array.push(Tag::MaxPower5v,TlvValue::from(device.max_power_5v()));
                array.push(Tag::MaxPower3v3,TlvValue::from(device.max_power_3v3()));

                array.push(Tag::SerialNumber, TlvValue::from(device.serial().clone()));
            } else {

                array.push(Tag::DeviceAddress,TlvValue::from(device.adr()));
                array.push(Tag::FirmwareVersion,TlvValue::from(device.fw_version().clone()));
                array.push(Tag::HardwareVersion,TlvValue::from(device.hw_version().clone()));
                array.push(Tag::SupportedSdbpVersion,TlvValue::from(device.protocol_version().clone()));
                array.push(Tag::MaxFrameSize,TlvValue::from(device.max_frame_size()));
                array.push(Tag::SerialNumber, TlvValue::from(device.serial().clone()));

            }
        }
        return tlv;
    }

    pub fn get_descriptor(devices : &Vec<Descriptor>, payload : &[u8]) -> Response{

        if payload.len() != 3 {
            trace!("get_descriptor - Invalid Length");
            return Response::new_error(Error::InvalidLength)
        }

        if payload[0] > 1 {
            trace!("get_descriptor - Invalid Parameter");
            return Response::new_error(Error::InvalidParameter)
        }


        let dev_adr = (((payload[1] as u16) << 8) & 0xFF00)  | (payload[2] as u16 & 0x00FF);
        let mode = payload[0] == 1;
        let result = ModApi::get_devices(devices,mode ,Some(dev_adr));

        let mut response = Response::new_empty_response();
        response.append_bytes(result.into_bytes().as_slice());
        response
    }

    pub fn get_device_list(devices : &Vec<Descriptor>, payload : &[u8]) -> Response {

        if payload.len() != 1 {
            trace!("get_device_list - Invalid Length");
            return Response::new_error(Error::InvalidLength)
        }

        if payload[0] > 1 {
            trace!("get_device_list - Invalid Parameter");
            return Response::new_error(Error::InvalidParameter)
        }
        let mode = if payload[0] == 1 { true} else {false};
        let mut response = Response::new_empty_response();

        let tlv = ModApi::get_devices(devices,mode, None);
        response.append_bytes(tlv.into_bytes().as_slice());
        response
    }
}

