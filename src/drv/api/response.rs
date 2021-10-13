use std::convert::TryInto;
use super::Command;
use super::error::Error as MdError;
use std::fmt;

pub struct Response {
    frame: Vec<u8>
}

impl Response {

    const HEADER_OFFSET : usize = 2;

    pub fn new_error(error : MdError) -> Response {

        let mut frame = Vec::<u8>::new();
        frame.extend_from_slice(&(Command::Error as u16).to_ne_bytes());
        frame.extend_from_slice(&error.to_bytes());
        Response {frame}
    }

    pub fn new_empty_response() -> Response{
        let mut frame = Vec::<u8>::new();
        frame.extend_from_slice(&(Command::Response as u16).to_ne_bytes());
        Response {frame}
    }

    pub fn get_op_id(&self) -> u16 {
        u16::from_ne_bytes(self.frame.as_slice()[0..2].try_into().unwrap())
    }

    pub fn get_payload(&self) -> &[u8]{
        &self.frame.as_slice()[Response::HEADER_OFFSET..self.frame.len()]
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.frame.as_slice()[..self.frame.len()]
    }

    pub fn append_vec(&mut self, payload : Vec<u8>) {
        self.frame.extend_from_slice(payload.as_slice());
    }

    pub fn append_bytes(&mut self, payload : & [u8]) {
        self.frame.extend_from_slice(payload);
    }

    pub fn append_byte(&mut self, payload : u8) {
        self.frame.push(payload);
    }

    pub fn from_bytes(input : &[u8]) -> Option<Response> {
        if input.len() < Response::HEADER_OFFSET {return None }
        let frame = Vec::from(input);
        Some(Response {frame})
    }
}

impl fmt::Debug for Response {

    fn fmt(&self, fmt : &mut fmt::Formatter) -> fmt::Result {

        let result = fmt.write_str(fmt::format(format_args!("{:?}",self.frame.as_slice())).as_str());
        result
    }

}