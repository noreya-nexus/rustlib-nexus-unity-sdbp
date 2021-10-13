use crossbeam_channel::{Sender};
use crate::drv::core::{PMsg, DeviceEventType};

#[derive(Debug,Clone,PartialEq)]
pub enum UdsEventState {
    CONNECTED,
    DISCONNECTED,
}

#[derive(Debug,Clone)]
pub struct IntDispatchEvent{

    pub id: u16,
    pub evt_type: UdsEventState,
    pub chn: Option<Sender<PMsg>>,
}

#[derive(Debug,Clone)]
pub struct IntUdsEvent{

    pub id: u16,
    pub evt_type: UdsEventState,
    pub chn: Option<Sender<PMsg>>,
}

#[derive(Debug,Clone)]
pub struct IntDeviceEvent {
    pub evt_type : DeviceEventType,
    pub chn: Option<Sender<PMsg>>,
    pub id: u16
}
