use super::error::Error;
use crate::drv::api::{Request, DRV_DEV_ADR, Command, TlvValue, Response, IntoBytes, Tag};

pub struct RequestBuilder{}

#[allow(dead_code)]
impl RequestBuilder {

    pub fn new() -> RequestBuilder {
        RequestBuilder{}
    }

    pub fn info(self) -> Request {
        Request::new_without_payload(DRV_DEV_ADR,Command::Info as u16)
    }

    pub fn get_device_list(self, short : bool) -> Request {
        Request::new_from_bytes(DRV_DEV_ADR,Command::GetDeviceList as u16,&[short as u8])
    }

    pub fn get_descriptor(self, short : bool, dev_adr : u16) -> Request {
        Request::new_from_bytes(DRV_DEV_ADR,Command::GetDescriptor as u16,&[short as u8, (dev_adr << 8) as u8, (dev_adr & 0xFF) as u8])
    }

    pub fn device_command(self, dev_adr : u16, payload : &[u8]) -> Request{
        Request::new_from_bytes(dev_adr as u16, Command::Device as u16, payload)
    }
}

pub struct ResponseBuilder{}

#[allow(dead_code)]
impl ResponseBuilder {

    pub fn new() -> ResponseBuilder {
        ResponseBuilder{}
    }

    pub fn tlv(tlv : TlvValue) -> Response {
        let mut result = Response::new_empty_response();
        result.append_bytes(tlv.into_bytes().as_slice());
        result
    }

    pub fn error(error : Error) {
        let mut value = TlvValue::new();
        value.push(Tag::ErrorValue,TlvValue::U16(error as u16));

    }
}

pub struct FrameBuilder {}

#[allow(dead_code)]
impl FrameBuilder {

    pub fn request() -> RequestBuilder{
        RequestBuilder::new()
    }

    pub  fn response() -> ResponseBuilder{
        ResponseBuilder::new()
    }
}

