use super::*;
use crossbeam_channel::{Sender};
use crate::util::ChannelPair;


pub struct ComHandler{
    reg_uds : Sender<IntUdsEvent>,
    reg_dev : Sender<IntDeviceEvent>,

    dev_src : Sender<PMsg>,
    client_src : Sender<PMsg>,
}

impl Clone for ComHandler {
    fn clone(&self) -> Self {
        ComHandler {dev_src : self.dev_src.clone(), reg_dev : self.reg_dev.clone(), reg_uds : self.reg_uds.clone(), client_src : self.client_src.clone()}
    }
}

impl ComHandler {
    pub fn new(reg_uds : Sender<IntUdsEvent>, reg_dev: Sender<IntDeviceEvent>,dev_src : Sender<PMsg>, client_src: Sender<PMsg>) -> ComHandler {
        ComHandler { reg_dev, reg_uds, dev_src, client_src}
    }

    fn send_client_evt(&self, evt: IntUdsEvent) {
        let _ = self.reg_uds.send(evt);
    }

    fn send_device_evt(&self, evt: IntDeviceEvent) {
        let _ = self.reg_dev.send(evt);
    }


    pub fn register_new_client(&self, client_id: u16) -> ChannelPair<PMsg> {
        let chn = crossbeam_channel::unbounded();
        let evt = IntUdsEvent { evt_type: UdsEventState::CONNECTED, id: client_id, chn: Some(chn.0) };
        self.send_client_evt(evt);
        ChannelPair::from_channels_to_single_pair(self.client_src.clone(),chn.1)
    }

    pub fn register_new_device(&self, device_id : u16) -> ChannelPair<PMsg> {
        let chn = crossbeam_channel::unbounded();
        let evt = IntDeviceEvent { evt_type: DeviceEventType::Connected, id: device_id, chn: Some(chn.0) };
        self.send_device_evt(evt);
        ChannelPair::from_channels_to_single_pair(self.dev_src.clone(),chn.1)
    }

    pub fn register_client(&self, client_id: u16, chn: Sender<PMsg>) {
        let evt = IntUdsEvent { evt_type: UdsEventState::CONNECTED, id: client_id, chn: Some(chn) };
        self.send_client_evt(evt);
    }

    pub fn register_device(&self, device_id: u16, chn: Sender<PMsg>) {
        let evt = IntDeviceEvent { evt_type: DeviceEventType::Connected, id: device_id, chn: Some(chn) };
        self.send_device_evt(evt);
    }

    pub fn unregister_client(&self, client_id: u16) {
        let evt = IntUdsEvent { evt_type: UdsEventState::DISCONNECTED, id: client_id, chn: None };
        self.send_client_evt(evt);
    }
    pub fn unregister_device(&self, device_id: u16) {
        let evt = IntDeviceEvent { evt_type: DeviceEventType::Disconnected, id: device_id, chn: None };
        self.send_device_evt(evt);
    }
}