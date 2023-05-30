use std::time::Duration;
use crate::util::*;
use crate::datatypes::*;
use crate::drv::core::PMsg;


pub type FuncDeviceHandler = fn (Descriptor, ChannelPair<ManagedThreadState>,ChannelPair<PMsg>, compatible_fw_major: u16, compatible_fw_minor: u16);

pub struct DeviceThread {
   handle : ManagedThreadHandle<()>,
}

impl DeviceThread{

    pub fn start(name:String,dev_chn: ChannelPair<PMsg>,desc : Descriptor, func : FuncDeviceHandler, compatible_fw_major: u16, compatible_fw_minor: u16 ) -> DeviceThread{

        let handle = spawn(name,move |ctl_chn| func(desc, ctl_chn,dev_chn, compatible_fw_major, compatible_fw_minor));
        DeviceThread { handle }
    }

    pub fn stop(&self,timout: Duration){
        let _  = self.handle.stop(timout);
    }
}