use std::time::Duration;
use std::os::unix::net::{UnixStream};
use std::io::{ErrorKind};
use crossbeam_channel::Sender;

use crate::util::*;
use crate::drv::api::{ModApi, Command, TlvValue, Tag, IntoBytes};
use crate::drv::core::{PMsg, SharedStats};
use crate::drv::api::Response;
use crate::drv::api::Error;

#[derive(Debug)]
pub enum UdsSessionStatus {
    DISCONNECTED
}

#[derive(Debug)]
pub struct UdsSessionResult {

    id : u16,
    status : UdsSessionStatus,
}

impl UdsSessionResult {

    pub fn disconnected(id : u16) -> UdsSessionResult {
           UdsSessionResult {status : UdsSessionStatus::DISCONNECTED,id}
    }

    pub fn get_status(&self) -> &UdsSessionStatus {
        &self.status
    }

    pub fn get_id(&self) -> u16{
        self.id
    }
}


pub type FuncUdsSessionTask = fn(ctl_pair : ChannelPair<ManagedThreadState>, nr : u16, data_pair : ChannelPair<PMsg>, stream : UnixStream, chn_result : Sender<UdsSessionResult>, stats : SharedStats);


pub struct UdsSessionHandler {
    handle : ManagedThreadHandle<()>
}

#[allow(unused_assignments,unused_variables)]
impl UdsSessionHandler {

    fn task(ctl_pair : ChannelPair<ManagedThreadState>, nr : u16, data_pair : ChannelPair<PMsg>, stream : UnixStream, chn_result : Sender<UdsSessionResult>, stats : SharedStats) {

        let stopped = false;
        let mut shared = stats;
        let mut _stats  = shared.read();
        let thread_name = std::thread::current().name().expect("Could not get tread name").to_string();
        debug!("Started {}",thread_name);
        trace!("Stats: {}",_stats);

        let mut reader = UnixStreamReader::from_unix_stream(stream,Some(Duration::from_millis(500)));

        loop {

            let raw_input = reader.read_msg();

            if let Err(err) = raw_input {

                match err.kind() {
                    ErrorKind::TimedOut => (),
                    ErrorKind::ConnectionAborted => { trace!("{} - Socket closed",thread_name); break },
                    _ => (),
                };
            } else if let Ok(input) = raw_input {

                //info!("Received:  {:?}", request.as_slice());
               let command = ModApi::parse(&input);
                trace!("Received Command: {:?}",command.0);
               let response =  match command {
                   (Some(Command::Info),Some(request)) => ModApi::info(_stats.get_version(), _stats.get_sdbpk_version()),
                   (Some(Command::GetDeviceList),Some(request))  => ModApi::get_device_list(_stats.get_devices(),request.get_payload()),
                   (Some(Command::GetDescriptor),Some(request))  => ModApi::get_descriptor(_stats.get_devices(),request.get_payload()),
                   (Some(Command::Device),Some(request))  => {

                       let _ = data_pair.tx().send(PMsg::create(nr, request.get_dev_id(),Ok(request.get_payload().to_vec())));
                       let result = data_pair.rx().recv_timeout(Duration::from_secs(5));

                       let response = match result {

                           Err(_err) => {
                               Response::new_error(Error::DeviceNotConnected)
                           },

                           Ok(value) => {
                               let mut response = Response::new_empty_response();
                               let mut tlv = TlvValue::new();

                               let msg = match value.get_msg() {
                                   None => {Response::new_error(Error::DeviceNotConnected)}
                                   Some(val) => {
                                       tlv.push(Tag::Response,TlvValue::from(val));
                                       response.append_bytes(tlv.into_bytes().as_slice());
                                       response
                                   }
                               };
                               msg
                           }
                       };
                       response
                    },
                    _ =>  Response::new_error(Error::UnknownCommand)
                };
                let _ = reader.write_msg(response.to_bytes());
            }

            if shared.shared().get_timestamp() > _stats.get_timestamp() {
                trace!("Changed Stats: \n{:?}", _stats);
                _stats = shared.read();
            }

            if !ctl_pair.rx().is_empty() {
                match ctl_pair.rx().recv() {
                    Ok(val) => {
                        if val == ManagedThreadState::STOPPED {
                            let _ = ctl_pair.tx().send(ManagedThreadState::OK);
                            break;
                        }
                    }
                    Err(_) => {
                        warn!("Could not receive thread state!");
                    }
                };
            }
        }
        debug!("Stopped {}",thread_name);
        let _ = chn_result.send(UdsSessionResult::disconnected(nr));
    }

    pub fn start(nr : u16, pair : ChannelPair<PMsg>, stream : UnixStream, chn_result: Sender<UdsSessionResult>, stats : SharedStats) -> UdsSessionHandler {


        let func = UdsSessionHandler::task;
        let handle = spawn(format!("client-{}",nr).to_string(),move |ctl_pair| func(ctl_pair, nr, pair, stream, chn_result, stats));
        UdsSessionHandler {handle}
    }

    pub fn stop(&self, dur : Duration) {
        let _ = self.handle.stop(dur);
    }

}