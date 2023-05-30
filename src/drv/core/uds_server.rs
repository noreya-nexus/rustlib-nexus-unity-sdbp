use std::collections::HashMap;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::{ create_dir_all};
use std::os::unix::fs::PermissionsExt;
use crate::util::*;
use crate::drv::core::*;
use std::io::{Error, ErrorKind};

use std::collections::hash_map::Iter;
use std::os::unix::net::UnixStream;

// pub struct UdsServerBuilder {
//
//     meta : Option<DrvMeta>,
//     com : Option<ComHandler>,
//     stats:  Option<SharedStats>,
// }

// impl UdsServerBuilder {
//
//
//
//     fn start(self) -> Result<UdsServer,Error> {
//
//         if self.meta.is_none() ||
//             self.stats.is_none() ||
//             self.com.is_none() {
//
//             Err(Error::new(ErrorKind::InvalidInput,"Missing parameter"))
//
//         }
//         Ok(UdsServer::start(self.meta.unwrap(),self.com.unwrap(),self.stats.unwrap()))
//
//     }
// }


pub struct UdsServer {
    path: String,
    handle : ManagedThreadHandle<()>,
}


pub struct ClientMap {
    map : HashMap<u16,UdsSessionHandler>,
    next : u16,
}



impl ClientMap {

    fn inc_next(&mut self) {
        self.next += 1;
        if self.next > 0xFFF {
            self.next = 0;
        }
    }

    pub fn new() -> ClientMap {
        ClientMap {next: 0, map: HashMap::new()}
    }

    pub fn get_next(&mut self) -> Result<u16,Error> {

        let mut result = Err(Error::new(ErrorKind::AddrNotAvailable, "No Address available"));

        if !self.map.contains_key(&self.next) {
            result = Ok(self.next | 0x1000);
            self.inc_next();
        } else {

            let mut addrs: Vec<_> = self.map.keys().collect();
            addrs.push(&0);
            addrs.push(&0xFFF);
            addrs.sort_by(| a, b | { a.cmp(b)});

            let mut i = 0;

            if addrs.len() == 1 {
                result = Ok((addrs[i] + 1) | 0x1000);
                self.next = addrs[i]+1;
            }

            while i + 1 < addrs.len() -1 {
                if addrs[i+1] - addrs[i] > 1 {
                    result = Ok((addrs[i] + 1) | 0x1000);
                    self.next = addrs[i]+1;
                    break;
                }
                i += 1;
            }
        }
        return result;
    }

    fn remove(&mut self, id: u16) {
        self.map.remove(&id);
    }

    fn insert(&mut self,id: u16, session: UdsSessionHandler) -> Option<UdsSessionHandler>{
        self.map.insert(id,session)
    }

    fn get(&mut self, id: u16) -> Option<&UdsSessionHandler> {
        self.map.get(&id)
    }

    pub fn iter(&self) -> Iter<'_, u16, UdsSessionHandler> {
        self.map.iter()
    }
}


impl UdsServer {

    fn uds_server(ctl_pair : ChannelPair<ManagedThreadState>, meta : DrvMeta, com : ComHandler,stats : SharedStats) {

        let mut stopped = false;
        let mut clients = ClientMap::new();
        let thread_name = std::thread::current().name().expect("Could not get tread name").to_string();

        info!("Started {}",thread_name);
        let path : PathBuf = PathBuf::from(format!("{}",meta.socket()));

        if !path.parent().expect("Could not find UDS socket path!").exists() {
            match create_dir_all(path.parent().expect("Could not create service runtime directory!")) {
                Err(_err) => {
                    error!("{:?}",_err.to_string());
                    return
                },
                _  => (),
            }
        }

        debug!("Listening @ {:?}",path);

        let chn_result = crossbeam_channel::unbounded();

        let uds = UnixDomainSocket::bind(path.clone()).expect("Could not bind UDS socket!");
        let _ = uds.get_listener().set_nonblocking(false);

        let meta = std::fs::metadata(path.clone()).expect("Could not read socket metadata!");
        let mut perm = meta.permissions();
        perm.set_mode(0o770);
        std::fs::set_permissions(path.clone(),perm).expect("Failed setting socket permissions!");

        for stream in uds.get_listener().incoming() {
            match stream {
                Ok(stream) => {
                    /* connection succeeded */
                    let id = clients.get_next().expect("Could not get new client id!");
                    let pair = com.register_new_client(id);
                    let handle = UdsSessionHandler::start(id, pair, stream, chn_result.0.clone(), stats.clone());
                    clients.insert(id, handle);

                }
               _ => trace!("Empty stream handle!"),
            }

            let result = chn_result.1.recv_timeout(Duration::from_millis(10));
            match result {
                Ok(value) => {
                    debug!("Unregister Client: {:?}",value);
                    com.unregister_client(value.get_id());
                    let t = clients.get(value.get_id());
                    match t  {
                        None => {
                           warn!("Could not stop UdsSessionHandler!")
                        }
                        Some(thread) => {
                            thread.stop(Duration::from_millis(1000));
                        }
                    };
                    clients.remove(value.get_id());
                },
                _ =>  trace!("Result is empty!"),
            };

            ManagedThreadUtil::is_stopped(&mut stopped, &ctl_pair);
            if stopped {
                break;
            }
        }

        for t in clients.iter() {
            t.1.stop(Duration::from_millis(1000));
        }

        info!("Stopped {}",thread_name);
    }

    pub fn start(meta: DrvMeta, com : ComHandler,stats : SharedStats) -> UdsServer {
        let path = meta.socket().clone();
        let handle = spawn( "UdsHandler".to_string(),move | ctl_pair| UdsServer::uds_server(ctl_pair, meta, com, stats));
        UdsServer { path: format!("{}",path), handle}
    }

    pub fn stop(&self, dur : Duration) {
        let _ = self.handle.stop(dur);
        let _ = UnixStream::connect(&self.path); // Trigger wakeup
    }
}

