use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use std::collections::HashMap;
use crossbeam_channel::{Sender, Receiver, RecvError};

use super::*;
use crate::util::*;
use crate::datatypes::BootloaderState;
use regex::Regex;

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

    fn detect(filter : &DeviceFilter<String>) -> Vec<(PathBuf,DeviceMeta)> {

        let mut detected_slots = vec![];

        let paths = fs::read_dir("/sys/class/sdbp").unwrap();
        for path in paths {
            match path {

                Ok(value) =>  {

                    let path = value.path();

                    let  id = match  sysfs::get_vendor_id(&path) {
                        Err(_err) => continue,
                        Ok(_value) => _value,
                    };

                    let rid = sysfs::get_rid(&path).unwrap();
                    let hash = sysfs::getid(&path).unwrap();

                    let meta = DeviceMeta::new(rid,hash);


                    let boot = sysfs::get_bootloader_state(&path);
                    match boot {
                        Ok(BootloaderState::NotSupported) | Ok(BootloaderState::Supported) => {
                            if filter.is_match(id) {
                                //error!("{:?} RID: {}, HASH: {}",path,rid,hash);
                                detected_slots.push((path,meta));
                            }
                        }
                        _ => {
                            trace!("Detected device in Bootloader mode.");
                        }
                    }
                },
                _ => (),
            };
        }
        detected_slots.sort();
        detected_slots
    }


    fn get_device_nr(path : &PathBuf) -> u16 {
        let regex = Regex::new("slot([0-9]*)").unwrap();
        let mut cap = regex.captures_iter(&path.to_str().unwrap());
        let test = cap.next().unwrap();
        return test[1].parse::<u16>().unwrap();
    }

    fn device_detection( ctl_pair : ChannelPair<ManagedThreadState>,sender : Sender<DeviceEvent>,filter : DeviceFilter<String>) {

        let mut stopped = false;
        let mut activated_slots : HashMap<PathBuf,DeviceMeta> = HashMap::new();

        while !stopped {
            ManagedThreadUtil::is_stopped(&mut stopped, &ctl_pair);

            let detected_slots = DeviceHandler::detect(&filter);

            let a = detected_slots.to_vec();

            for device in a {

                match activated_slots.get_mut(&device.0) {
                    None => {
                        let nr = DeviceHandler::get_device_nr(&device.0);
                        sender.send(DeviceEventBuilder::generate(DeviceEventType::Connected, nr, &device.0,false)).unwrap();
                        debug!("Connected Device: {:?}", &device.0);
                        activated_slots.insert(device.0, device.1);
                    },

                    Some(value) => {
                        if value.rid == device.1.rid && value.hash != device.1.hash {
                            let nr = DeviceHandler::get_device_nr(&device.0);
                            value.update_hash(device.1.hash);
                            debug!("Updated Device: {:?}", &device.0);
                            sender.send(DeviceEventBuilder::generate(DeviceEventType::Updated, nr, &device.0,false)).unwrap();

                        }
                    }
                }
            }

            let mut removed_slots = vec![];

            for (slot_path, meta) in &activated_slots {
                if !&detected_slots.contains(&(slot_path.clone(), meta.clone())) {
                    debug!("Disconnected Device: {:?} {:?}", &slot_path, meta.rid);

                    let nr = DeviceHandler::get_device_nr(&slot_path);
                    sender.send(DeviceEventBuilder::generate(DeviceEventType::Disconnected, nr,&slot_path,false)).unwrap();
                    removed_slots.push(slot_path.clone());
                }
            }

            for dev in removed_slots {
                activated_slots.remove(&dev);
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    pub fn start(filter: DeviceFilter<String>, rx_chn : Receiver<DeviceEvent>, tx_chn : Sender<DeviceEvent>) -> DeviceHandler{

        //let (sender,receiver) = crossbeam_channel::unbounded();
        let handle = spawn("Device Detection".to_string(), move |stopped| DeviceHandler::device_detection(stopped,tx_chn,filter));
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