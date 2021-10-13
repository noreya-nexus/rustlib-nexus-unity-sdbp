use std::convert::TryInto;

pub struct Request {
    frame: Vec<u8>
}

impl Request {
    const HEADER_OFFSET: usize = 4;

    pub fn new_without_payload(dev_adr: u16, op_id: u16, ) -> Request {
        let mut frame = Vec::<u8>::new();
        frame.extend_from_slice(&dev_adr.to_ne_bytes());
        frame.extend_from_slice(&op_id.to_ne_bytes());
        Request { frame }
    }

    pub fn new_from_vec(dev_adr: u16, op_id: u16, payload: Vec<u8>) -> Request {
        let mut frame = Vec::<u8>::new();
        frame.extend_from_slice(&dev_adr.to_ne_bytes());
        frame.extend_from_slice(&op_id.to_ne_bytes());
        frame.extend_from_slice(payload.as_slice());
        Request { frame }
    }

    pub fn new_from_bytes(dev_adr: u16, op_id: u16, payload: &[u8]) -> Request {
        let mut frame = Vec::<u8>::new();
        frame.extend_from_slice(&dev_adr.to_ne_bytes());
        frame.extend_from_slice(&op_id.to_ne_bytes());
        frame.extend_from_slice(payload);
        Request { frame }
    }

    pub fn new_from_byte(dev_adr: u16, op_id: u16, payload: u8) -> Request {
        let mut frame = Vec::<u8>::new();
        frame.extend_from_slice(&dev_adr.to_ne_bytes());
        frame.extend_from_slice(&op_id.to_ne_bytes());
        frame.push(payload);
        Request { frame }
    }

    pub fn get_dev_id(&self) -> u16 {
        u16::from_ne_bytes(self.frame.as_slice()[0..2].try_into().unwrap())
    }

    pub fn get_op_id(&self) -> u16 {
        u16::from_ne_bytes(self.frame.as_slice()[2..4].try_into().unwrap())
    }

    pub fn get_payload(&self) -> &[u8] {
        &self.frame.as_slice()[Request::HEADER_OFFSET..self.frame.len()]
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.frame.as_slice()[..self.frame.len()]
    }

    pub fn append_vec(&mut self, payload: Vec<u8>) {
        self.frame.extend_from_slice(payload.as_slice());
    }

    pub fn append_bytes(&mut self, payload: &[u8]) {
        self.frame.extend_from_slice(payload);
    }

    pub fn append_byte(&mut self, payload: u8) {
        self.frame.push(payload);
    }

    pub fn from_bytes(input: &[u8]) -> Option<Request> {
        if input.len() < Request::HEADER_OFFSET { return None }
        let frame = Vec::from(input);
        Some(Request { frame })
    }
}
