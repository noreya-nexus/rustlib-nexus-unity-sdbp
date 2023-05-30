use std::fs::{File, OpenOptions};
use std::path::{PathBuf};
use std::io::{ErrorKind, Read, Write};


pub struct DeviceHandle {
    dev_file: File,
    dev_file_path: PathBuf,

}

impl DeviceHandle {
    pub fn new(slot_path: &PathBuf) -> Option<DeviceHandle> {
        let mut path = PathBuf::from(slot_path);

        path.push("dev");
        let file = OpenOptions::new().write(true).read(true).open(slot_path);

        match file {
            Ok(value) => return Some(DeviceHandle { dev_file: value, dev_file_path: slot_path.clone() }),
            Err(error) => {
                trace!("{:?}",error);
            }
        }
        return None;
    }

    pub fn read(&mut self, buf: &mut Vec<u8>) -> Result<usize, std::io::Error> {
        if self.dev_file_path.as_path().exists() {
            let result = self.dev_file.read(&mut buf.as_mut_slice());
            result
        } else {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "Device disconnected"));
        }
    }

    pub fn write(&mut self, buf: Vec<u8>) -> Result<usize, std::io::Error> {
        if self.dev_file_path.as_path().exists() {
            let result = self.dev_file.write(buf.as_slice());
            match result {
                Err(err) => {
                    const ENODEV: i32 = 19;
                    if err.raw_os_error() == Some(ENODEV) {
                        return Err(std::io::Error::new(ErrorKind::NotConnected, "Device disconnected"));
                    }
                    return Err(err);
                }
                val => { val }
            }
        } else {
            return Err(std::io::Error::new(ErrorKind::NotConnected, "Device disconnected"));
        }
    }
}