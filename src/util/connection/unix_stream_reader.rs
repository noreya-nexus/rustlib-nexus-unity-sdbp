use std::os::unix::net::{UnixStream};
use std::io::{Error,ErrorKind};
use std::time::Duration;
use std::io::Read;
use std::io::Write;
use super::*;

pub struct UnixStreamReader {
    stream : UnixStream,
}

impl UnixStreamReader {

    pub fn from_unix_stream(stream : UnixStream,timeout : Option<Duration>) -> UnixStreamReader {

        stream.set_read_timeout(timeout).expect("Cannot set read timeout on uds socket");
        UnixStreamReader {stream}
    }
}

impl Connection for UnixStreamReader {

    fn read_msg(&mut self) -> Result<Vec<u8>,Error> {

        let mut length_buffer: [u8;4] = [0; 4];
        let mut buffer : [u8;4096] = [0; 4096];

        //Read Header
        let status = self.stream.read(&mut length_buffer);

        if let Ok(len) = status {
            if len == 0 {  return Err(Error::new(ErrorKind::ConnectionAborted, "Socket closed"));}
        }

        if let Err(err) = status {
            if err.kind() ==  ErrorKind::WouldBlock { return Err(Error::new(ErrorKind::TimedOut,"No data received")); }
        }

        let header = u32::from_ne_bytes(length_buffer);

        if header > 4096 {
            let mut msg: String = "Header length out of range: ".to_owned();
            let msg_combined: String = header.to_string();
            msg.push_str(&msg_combined);
            return Err(Error::new(ErrorKind::InvalidData,msg))
        }

        let status = self.stream.read(&mut buffer);

        if let Ok(len) = status {
            if len == 0 {  return Err(Error::new(ErrorKind::ConnectionAborted, "Socket closed"));}
        } else if let Err(err) = status {
            if err.kind() ==  ErrorKind::WouldBlock { return Err(Error::new(ErrorKind::TimedOut,"No data received")); }
        }

        Ok(Vec::from(&buffer[..header as usize]))
    }

    fn write_msg(&mut self, msg : &[u8],) -> Result<(),Error> {

        let len = (msg.len() as u32).to_ne_bytes();

        if let Err(err) = self.stream.write_all(&len) { return Err(err)}
        if let Err(err) = self.stream.flush() {return  Err(err)}
        if let Err(err) = self.stream.write_all(&msg) { return Err(err)}
        if let Err(err) = self.stream.flush() {return Err(err)}

        Ok(())
    }
}
