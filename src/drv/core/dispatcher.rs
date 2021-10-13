#![allow(dead_code)]

use crossbeam_channel::{Receiver, Sender, Select};
use std::time::{Duration};
use std::collections::HashMap;

use super::*;
use crate::util::*;

struct DispatcherLogic {
    rt_devices : HashMap<u16,Sender<PMsg>>,
}

pub struct DispatcherSettings {

    uds_evt: Receiver<IntUdsEvent>,
    dev_evt: Receiver<IntDeviceEvent>,

    rx_client : Receiver<PMsg>,
    rx_device : Receiver<PMsg>,
}

pub struct Dispatcher {
    handle : ManagedThreadHandle<()>,
    com : ComHandler,
}


impl DispatcherLogic {

    fn new() -> DispatcherLogic{
        DispatcherLogic{rt_devices : HashMap::new()}
    }

    pub fn register_device(&mut self, evt : IntDeviceEvent) {
        match evt.evt_type {
            DeviceEventType::Connected  => {
                self.rt_devices.insert(evt.id,evt.chn.unwrap());
            },
            DeviceEventType::Disconnected => {
                self.rt_devices.remove(&evt.id);
            },
            _ => (),
        };
    }

    pub fn register_client(&mut self, evt : IntUdsEvent) {

        match evt.evt_type {
            UdsEventState::CONNECTED => self.rt_devices.insert(evt.id,evt.chn.unwrap()),
            UdsEventState::DISCONNECTED => self.rt_devices.remove(&evt.id),
        };
    }

    pub fn register(&mut self, evt : IntDispatchEvent) {

        match evt.evt_type {
            UdsEventState::CONNECTED => self.rt_devices.insert(evt.id,evt.chn.unwrap()),
            UdsEventState::DISCONNECTED => self.rt_devices.remove(&evt.id),
        };
    }


    pub fn send_to_device(&self, msg : PMsg) {
        let dst = self.rt_devices.get(&msg.get_dst());
        match dst.unwrap().send(msg){
            Err(_err) => error!("{:?}",_err),
            _=> (),
        };
    }

    pub fn send_to_client(&self, msg : PMsg) {

        let dst = self.rt_devices.get(&msg.get_dst());
        let result = dst.unwrap().send(msg);
        match result {
            Err(_err) => error!("{:?}",_err),
            _ => (),
        }
    }

    fn task(ctl_pair: ChannelPair<ManagedThreadState>, settings : DispatcherSettings) {

        let mut stopped = false;
        let mut sel = Select::new();
        let reg_device = sel.recv(&settings.dev_evt);
        let reg_client = sel.recv(&settings.uds_evt);
        let rx_from_devices = sel.recv(&settings.rx_device);
        let rx_from_clients = sel.recv(&settings.rx_client);
        let ctl = sel.recv(ctl_pair.rx());

        let mut dispatch = DispatcherLogic::new();

        debug!("Started Dispatcher Thread");
        while !stopped {
            ManagedThreadUtil::is_stopped(&mut stopped,&ctl_pair);
            let op = sel.select();
            match op.index() {

                i if i == reg_device => {

                    let value = op.recv(&settings.dev_evt);
                    trace!("Register Device");
                    match  value {
                        Ok(val) => {
                            trace!("Register Device: {:?}",val);
                            dispatch.register_device(val)
                        }
                        _ => (),
                    };
                } ,
                i if i == reg_client => {
                    let value = op.recv(&settings.uds_evt);
                    match value {
                        Ok(val) => dispatch.register_client(val),
                        _ => (),
                    };
                },
                i if i == rx_from_clients => {
                    let value = op.recv(&settings.rx_client);
                    match value {
                       Ok(val) => dispatch.send_to_device(val),
                        _ => (),
                    };

                },
                i if i == rx_from_devices => {

                    //debug!("Timediff: {} ns",test.elapsed().as_nanos());
                    let value = op.recv(&settings.rx_device);
                    match value {
                        Ok(val) => dispatch.send_to_client(val),
                        _ => (),
                    }
                },
                i if i == ctl => {
                    let cmd = op.recv(&ctl_pair.rx()).unwrap();
                    match cmd {
                        ManagedThreadState::STOPPED => {
                            let _ = ctl_pair.tx().send(ManagedThreadState::OK);
                            stopped = true;
                        },
                        _ => (),
                    };
                }

                _ => (),
            }
        }
        info!("Stopped Dispatcher");
    }
}

impl Dispatcher {

    pub fn start() -> Dispatcher {

        let chn_dev_data = crossbeam_channel::unbounded();
        let chn_client_data = crossbeam_channel::unbounded();

        let chn_uds_evt = crossbeam_channel::unbounded();
        let chn_dev_evt = crossbeam_channel::unbounded();

        let settings = DispatcherSettings { rx_client : chn_client_data.1, rx_device : chn_dev_data.1, uds_evt : chn_uds_evt.1, dev_evt : chn_dev_evt.1 };
        //let handle = DispatcherHandle {uds_evt : chn_uds_evt.0, dev_evt : chn_dev_evt.0,device : chn_dev_data.0,client: chn_client_data.0};
        let com = ComHandler::new(chn_uds_evt.0,chn_dev_evt.0,chn_dev_data.0,chn_client_data.0);
        let handle = spawn( "Dispatcher".to_string(),move  |ctl_pair| DispatcherLogic::task(ctl_pair,settings));
        Dispatcher { handle, com}
    }

    pub fn stop(&self,dur : Duration) {
        let _ = self.handle.stop(dur);
    }

    pub fn get_com(&self) -> ComHandler {
        self.com.clone()
    }
}