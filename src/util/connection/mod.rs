pub mod unix_stream_reader;
pub mod client;

use std::io::{Error};
pub use unix_stream_reader::*;

pub trait Connection {
    fn read_msg(&mut self) -> Result<Vec<u8>,Error>;
    fn write_msg(&mut self, msg : &[u8],) -> Result<(),Error>;
}

#[cfg(test)]
mod uds_test {
    use std::path::PathBuf;
    use std::thread::{spawn, sleep};
    use std::os::unix::net::UnixStream;
    use std::time::Duration;
    use crate::util::{UnixStreamReader, Connection, UnixDomainSocket};
    use crate::util::connection::client::uds_client::UdsClient;


    const TEST_SOCKET : &str = "/tmp/test-uds.socket";

    struct TestServer{}

    #[allow(dead_code)]
    impl TestServer {

        pub fn new() -> TestServer {
            TestServer{}
        }

        fn loopback(stream : UnixStream) {

            let mut handler = UnixStreamReader::from_unix_stream(stream,None);

            loop {
                let response = handler.read_msg().unwrap();
                let _ = handler.write_msg(response.as_slice());
            }
        }

        pub fn run(self) {

            let path : PathBuf = PathBuf::from(format!("{}",TEST_SOCKET));

            if path.exists() {
                std::fs::remove_file(path.clone()).unwrap()
            }

            let uds = UnixDomainSocket::bind(path).expect("Fuck");
            let _ = uds.get_listener().set_nonblocking(true);

            for stream in uds.get_listener().incoming() {
                match stream {
                    Ok(stream) => {spawn(move || TestServer::loopback(stream));},
                    _ => (),
                };
            }
        }
    }

    #[test]
    fn connection_test() {

        let server = TestServer::new();
        spawn(move || server.run());
        sleep(Duration::from_secs(2));

        let mut client = UdsClient::from_string(TEST_SOCKET.to_string()).unwrap();
        client.connect().unwrap();

        let frame : Vec<u8> = vec![0x01,0x02,0x03];
        for _ in 0..100 {
            let _result = client.write_msg(frame.as_slice()).unwrap();
            let response = client.read_msg().unwrap();
            assert_eq!(frame, response, "Response is wrong");
        }
    }


}
