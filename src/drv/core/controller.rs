use std::time::Duration;
use std::collections::HashMap;
use crossbeam_channel::{Receiver, Select};


use crate::util::*;
use super::*;

pub struct Controller {
    handle : ManagedThreadHandle<()>
}

impl Controller {

    pub fn start_virtual_device(name: String, id: u16,  com : &mut ComHandler,shared : SharedStats, handle_func : FuncVirtualDeviceHandler) -> VirtualDeviceThread{
        let id = id | 0x2000;
        let pair = com.register_new_device(id);
       VirtualDeviceThread::start(name, id ,pair,handle_func,shared)
    }

    fn handle_evt(evt :  DeviceEvent, map : &mut HashMap<u16,DeviceThread>, com : &mut ComHandler, shared : &mut SharedStats, handle_func : FuncDeviceHandler, compatible_fw_major: u16, compatible_fw_minor: u16) {

        let mut stats = shared.read();

        if evt.evt_type == DeviceEventType::Connected {
            trace!("{:?}", evt);
            if map.get(&evt.id).is_some() {
                error!("Slot {} already registered!", evt.id);
                return;
            }

            let pair = com.register_new_device(evt.id);

            if !evt.is_virtual {
                let desc = match detection::sysfs::get_descriptor(&evt.path) {
                    Ok(value) => {
                        //debug!(value);
                        value
                    },
                    Err(_err) => {
                        error!("Error: {:?}", _err);
                        return
                    }
                };

                let device = DeviceThread::start(format!("dev-slot-{}", evt.id), pair, desc.clone(), handle_func, compatible_fw_major, compatible_fw_minor);
                map.insert(evt.id, device);
                if !evt.is_virtual {
                    stats.get_devices().push(desc);
                }

                match shared.write(stats,Some(Duration::from_secs(100))) {
                    Ok(_) => debug!("Write was successful"),
                    Err(_) => error!("Write to shared stats failed!")
                };
            }

        }
        else if evt.evt_type == DeviceEventType::Updated {
            trace!("{:?}",evt);

            if !evt.is_virtual {
                let desc = match detection::sysfs::get_descriptor(&evt.path) {
                    Ok(value) => {
                        //debug!(value);
                        value
                    },
                    Err(_err) => {
                        error!("Error: {:?}", _err);
                        return
                    }
                };

                let dev = stats.get_devices();

                let mut idx = -1;
                for i in 0..dev.len() {
                    if dev[i].adr() == evt.id {
                        idx = i as isize;
                        break;
                    }
                }

                if idx != -1 {
                    dev[idx as usize] = desc;
                }

                match shared.write(stats,Some(Duration::from_secs(100))) {
                    Ok(_) => debug!("Write was successful"),
                    Err(_) => error!("Write to shared stats failed!")
                };
            }

        } else if evt.evt_type == DeviceEventType::Disconnected {
            trace!("{:?}",evt);

            let t = map.get(&evt.id);

            let list = stats.get_devices();
            let mut found = false;
            for i in 0..list.len() {
                match list.get(i) {
                    None => {
                        info!("Could not find device in connected list")
                    }
                    Some(val) => {
                        if val.adr() == evt.id {
                            info!("Removed device from slot {}", evt.id);
                            list.remove(i);
                            found=true;
                            break;
                        }
                    }
                };
            }
            if !found {
                debug!("Device not found for removal {} (not handled by this driver)", evt.id);
                return;
            }
            match shared.write(stats,Some(Duration::from_secs(100))) {
               Ok(_) => (),
               Err(_) => error!("Write to shared stats failed")
            };
            //info!("{}",shared.read());
            match t {
                None => {
                    warn!("Could not stop thread!");
                }
                Some(thread) => {
                    thread.stop(Duration::from_millis(1000));
                }
            };
            com.unregister_device(evt.id);
            map.remove(&evt.id);
        }
    }

    fn task(ctl_pair : ChannelPair<ManagedThreadState>, mut com : ComHandler, chn_devt : Receiver<DeviceEvent>,stats : SharedStats, func_handle : FuncDeviceHandler, compatible_fw_major: u16, compatible_fw_minor: u16){

        let mut shared = stats;
        let mut stopped = false;
        let mut device_map : HashMap<u16,DeviceThread> = HashMap::new();

        let mut sel = Select::new();
        let op_evt = sel.recv(&chn_devt);
        let op_ctl = sel.recv(ctl_pair.rx());

        info!("Started Controller");

        while stopped == false {
            ManagedThreadUtil::is_stopped(&mut stopped, &ctl_pair);

            let op = sel.select();

            match op.index() {

                i if i == op_evt => {
                    let event = op.recv(&chn_devt);
                    match event {
                        Ok(value) => Controller::handle_evt(value, &mut device_map, &mut com, &mut shared, func_handle, compatible_fw_major, compatible_fw_minor),
                        Err(err) => error!("Controller error: {:?}", err)
                    }
                },
                i if i == op_ctl => {
                    let cmd = op.recv(&ctl_pair.rx());
                    match cmd {
                        Ok(val) => {
                            if val == ManagedThreadState::STOPPED {
                                let _ = ctl_pair.tx().send(ManagedThreadState::OK);
                                stopped = true;
                            }
                        }
                        Err(err) => {
                            error!("Controller error: {:?}", err)
                        }
                    };
                },
                _ => (),
            }

        }

        for (_nr , t) in device_map {
            t.stop(Duration::from_millis(1000));
        }
        info!("Stopped Controller");
    }

    pub fn start(com : ComHandler, chn_devt : Receiver<DeviceEvent>, stats : SharedStats, handle_func :  FuncDeviceHandler, compatible_fw_major: u16, compatible_fw_minor: u16 ) -> Controller {

        let handle = spawn("Controller".to_string(),move |ctl_pair |  Controller::task(ctl_pair,com,chn_devt,stats,handle_func, compatible_fw_major, compatible_fw_minor));
        Controller {handle}
    }

    pub fn stop(&self, dur : Duration) {
        let _ = self.handle.stop(dur);
    }
}