use std::path::{PathBuf};
use std::os::unix::net::{UnixListener};
use std::io::ErrorKind;

pub struct UnixDomainSocket {
    path: PathBuf,
    listener : UnixListener
}

impl UnixDomainSocket {

    pub fn bind(path : PathBuf) -> Result<UnixDomainSocket,std::io::Error>{

        let listener = UnixListener::bind(&path);

        match listener {
            Ok(value) => return Ok(UnixDomainSocket{path,listener:value}),
            Err(_) => return Err(std::io::Error::new(ErrorKind::ConnectionRefused,"Cannot bind socket")),
        };
    }

    pub fn get_listener(&self) -> &UnixListener {
        &self.listener
    }
}

impl Drop for UnixDomainSocket {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path).expect("Could not cleanup UDS path");
    }
}