use std::io::Error;

pub mod custom;
pub mod core;

pub trait SdbpResponse : Sized {
    fn from_raw(raw: Vec<u8>) -> Result<Self,Error>;
}
