use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Read, Write};

pub struct DeviceHandle{

    dev_file: File,

}

impl DeviceHandle {

    pub fn new(slot_path : &PathBuf)-> Option<DeviceHandle> {


        let mut path = PathBuf::from(slot_path);

        path.push("dev");
        let file = OpenOptions::new().write(true).read(true).open(slot_path);

        match file {
            Ok(value) => return Some(DeviceHandle{dev_file: value}),
            Err(error) => {
                trace!("{:?}",error);
            }
        }
        return None;
    }

    pub fn read(&mut self,buf: &mut Vec<u8>)-> Result<usize,std::io::Error> {

        let result = self.dev_file.read(&mut buf.as_mut_slice());
        result
    }

    pub fn write(&mut self, buf: Vec<u8>) -> Result<usize,std::io::Error> {

        let result = self.dev_file.write(buf.as_slice());
        result
    }
}