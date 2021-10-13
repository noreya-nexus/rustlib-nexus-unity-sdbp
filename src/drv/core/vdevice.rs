use std::time::Duration;
use crate::util::*;
use crate::drv::core::{PMsg, SharedStats};


pub type FuncVirtualDeviceHandler = fn (u16, ChannelPair<ManagedThreadState>,ChannelPair<PMsg>,SharedStats);

pub struct VirtualDeviceThread {
    handle : ManagedThreadHandle<()>,
}

impl VirtualDeviceThread{

    pub fn start(name:String,id : u16,dev_chn: ChannelPair<PMsg>,func : FuncVirtualDeviceHandler, shared : SharedStats ) -> VirtualDeviceThread{

        let handle = spawn(name,move |ctl_chn| func(id,ctl_chn,dev_chn, shared));
        VirtualDeviceThread { handle }
    }

    pub fn stop(&self,timout: Duration){
        let _  = self.handle.stop(timout);
    }
}