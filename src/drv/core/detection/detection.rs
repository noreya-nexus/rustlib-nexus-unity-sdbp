use std::path::PathBuf;
use std::time::Duration;
use crossbeam_channel::{Sender, Receiver, RecvError};
use mio::{Events, Interest, Poll, Token};

use crate::util::*;
use regex::Regex;
use udev::MonitorBuilder;
use crate::drv::core::udevhandler::{DEVICE_CLASS, event_happened, init_devices};

#[derive(Debug,Clone,Eq,Ord, PartialOrd, PartialEq)]
pub struct DeviceMeta  {
    rid : u64,
    hash: u32,
}

impl DeviceMeta {

    pub fn new(rid : u64, hash : u32) -> DeviceMeta {
        DeviceMeta{rid,hash}
    }

    pub fn hash(&self) -> u32 {
        self.hash
    }

    pub fn update_hash(&mut self, new_hash : u32) {
        self.hash = new_hash
    }

    pub fn rid(&self) -> u64 {
        self.rid
    }

}

#[derive(Debug,Clone)]
pub struct DeviceFilter<T> {

    supported_device : Vec<T>,
}

impl<T: Clone + std::cmp::PartialEq + std::fmt::Debug > DeviceFilter<T> {

    pub fn new() -> DeviceFilter<String> {

        let foo  = vec!["test".to_string(),"test".to_string()];
        DeviceFilter {supported_device: foo}
    }

    pub fn is_match(&self , value: T) -> bool {
        for entry in &self.supported_device {

            if *entry == value {
                trace!("Filter matches {:?}",entry);
                return true
            }
        }
        false
    }

    pub fn add(&mut self, value: T) {
        self.supported_device.push(value);
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.supported_device.clear();
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum DeviceEventType {

    Connected,
    Updated,
    Disconnected,
}

#[derive(Debug,Clone)]
pub struct DeviceEvent {
    pub evt_type : DeviceEventType,
    pub is_virtual: bool,
    pub id : u16,
    pub path: PathBuf,
}

pub(crate) struct DeviceEventBuilder;
impl DeviceEventBuilder {

    pub fn generate(evt_type : DeviceEventType, id : u16, path : &PathBuf, is_virtual: bool) -> DeviceEvent{

        let _path = path.clone();

        DeviceEvent {
            evt_type,
            id,
            is_virtual,
            path: _path,
        }
    }
}

pub struct DeviceHandler {
    handle : ManagedThreadHandle<()>,
    queue_rcv: Receiver<DeviceEvent>
}

impl DeviceHandler {

    pub(crate) fn get_device_nr(path : &PathBuf) -> u16 {
        let regex = Regex::new("slot([0-9]*)").expect("Could not build regex");
        let mut cap = regex.captures_iter(&path.to_str().expect("Could not get device path"));
        let test = cap.next().expect("Could not iterate through devices");
        return test[1].parse::<u16>().expect("Could not convert to u16");
    }

    fn device_detection( ctl_pair : ChannelPair<ManagedThreadState>,sender : Sender<DeviceEvent>, device_filter : DeviceFilter<String>) {
        let thread_name = std::thread::current().name().expect("Could not get tread name").to_string();
        let mut stopped = false;
        // let mut activated_slots : HashMap<PathBuf,DeviceMeta> = HashMap::new();

        init_devices(&sender, &device_filter);

        let filter = MonitorBuilder::new().unwrap();
        let filter = filter.match_subsystem(DEVICE_CLASS).unwrap();
        let mut udev_socket = filter.listen().unwrap();
        let mut poll = Poll::new().expect("Could not create poll instance");
        let mut events = Events::with_capacity(1024);

        poll.registry().register(
            &mut udev_socket,
            Token(0),
            Interest::READABLE | Interest::WRITABLE,
        ).expect("Could not register poll events");

        while !stopped {
            match poll.poll(&mut events, Some(Duration::from_millis(500))) {
                Ok(_) => {}
                Err(err) => {
                    error!("Poll Error: {:?}", err);
                }
            }

            for event in &events {
                if event.token() == Token(0) && event.is_writable() {
                    for event in udev_socket.iter() {
                        event_happened(&sender, event, &device_filter);
                    }
                }
            }
            ManagedThreadUtil::is_stopped(&mut stopped, &ctl_pair);
        }
        info!("Stopped {}",thread_name);
    }

    pub fn start(filter: DeviceFilter<String>, rx_chn : Receiver<DeviceEvent>, tx_chn : Sender<DeviceEvent>) -> DeviceHandler{

        //let (sender,receiver) = crossbeam_channel::unbounded();
        let handle = spawn("DeviceDetection".to_string(), move |stopped| DeviceHandler::device_detection(stopped,tx_chn,filter));
        DeviceHandler{handle, queue_rcv: rx_chn}
    }


    pub fn recv(&self) -> Result<DeviceEvent,RecvError>{
        return self.queue_rcv.recv();
    }


    #[allow(dead_code)]
    pub fn stop(&self,dur : Duration) {
        let _ = self.handle.stop(dur);
    }
}