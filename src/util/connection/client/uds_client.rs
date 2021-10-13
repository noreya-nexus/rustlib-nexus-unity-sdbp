use std::path::{PathBuf};
use std::io::Error;
use std::io::ErrorKind;
use std::os::unix::net::UnixStream;
use crate::util::UnixStreamReader;
use crate::util::Connection;

pub struct UdsClient{
    path : PathBuf,
    reader : Option<UnixStreamReader>,
}

#[allow(dead_code)]
impl UdsClient {

    pub fn from_string(path : String) -> Result<UdsClient,Error>{
        UdsClient::from_path(PathBuf::from(path))
    }

    pub fn from_path(path: PathBuf) -> Result<UdsClient,Error>{
        let result;
        if !path.exists() {
            result = Err(Error::new(ErrorKind::NotFound,format!("Not found : {:?}",path)));
        }
        else {
            result = Ok(UdsClient{path,reader: None});
        }
        result
    }
    pub fn connect(&mut self) -> Result<(),Error> {

        let test = self.path.to_str().unwrap();

        let stream = match UnixStream::connect(test) {
            Ok(value) => value,
            Err(_) => return Err(Error::new(ErrorKind::ConnectionRefused, "Cannot connect to socket")),
        };
        let tmp = UnixStreamReader::from_unix_stream(stream,None);
        self.reader = Some(tmp);
        Ok(())
    }
}

impl Connection for UdsClient {
    fn read_msg(&mut self) -> Result<Vec<u8>, Error> {
        match &mut self.reader {
            None => Err(Error::new(ErrorKind::NotConnected,"Client ist not connected")),
            Some(value) => value.read_msg(),
        }
    }

    fn write_msg(&mut self, msg: &[u8]) -> Result<(), Error> {
        match &mut self.reader {
            None => Err(Error::new(ErrorKind::NotConnected,"Client ist not connected")),
            Some(value) => value.write_msg(msg),
        }
    }
}