use std::time::Duration;
use crate::util::*;
use crate::datatypes::*;
use crate::drv::core::PMsg;


pub type FuncDeviceHandler = fn (Descriptor, ChannelPair<ManagedThreadState>,ChannelPair<PMsg>);

pub struct DeviceThread {
   handle : ManagedThreadHandle<()>,
}

impl DeviceThread{

    pub fn start(name:String,dev_chn: ChannelPair<PMsg>,desc : Descriptor, func : FuncDeviceHandler ) -> DeviceThread{

        let handle = spawn(name,move |ctl_chn| func(desc, ctl_chn,dev_chn));
        DeviceThread { handle }
    }

    pub fn stop(&self,timout: Duration){
        let _  = self.handle.stop(timout);
    }
}