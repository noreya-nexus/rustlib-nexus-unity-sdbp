use crate::datatypes::Descriptor;
use crate::util::*;
use crate::drv::core::*;

use std::fs::File;
use std::io::Read;
use crate::{err_slot, info_slot};

pub struct NotificationHandler {}

impl NotificationHandler {
    pub fn task(desc: Descriptor, ctl_pair: ChannelPair<ManagedThreadState>, dev_pair: ChannelPair<PMsg>) {
        let mut stopped = false;

        let path: String = format!("/sys/devices/virtual/sdbp/slot{}/notification", desc.adr());

        info_slot!(&path, "Started notification handler");

        while !stopped {
            ManagedThreadUtil::is_stopped(&mut stopped, &ctl_pair);
            if stopped {
                break;
            }

            let f = File::open(&path);
            let mut fh = match f {
                Ok(value) => value,
                Err(err) => {
                    panic!("Could not open notification file: {}: {:?} ", path, err);
                }
            };

            let mut buffer = Vec::new();
            let result = fh.read_to_end(&mut buffer);
            match result {
                Ok(_) => {
                    let notification_string = String::from_utf8_lossy(&buffer);
                    //info!("Decoded notification: {:?}", notification_string);
                    //info!("Decoded notification hex: {:x?}", notification_string);
                    //info!("Decoded notification length: {:?}", notification_string.len());
                    let notification_string = notification_string.strip_prefix("0x").expect("Decoding notification failed");
                    let notification_string = notification_string.strip_suffix("\0").expect("Decoding notification failed");
                    let notification_string = hex::decode(notification_string).expect("Decoding notification failed");
                    debug!("Decoded hex notification: {:x?}", notification_string);
                    let msg = PMsg::create(0, 0, Ok(notification_string));
                    match dev_pair.tx().send(msg) {
                        Ok(_) => {}
                        Err(err) => {
                            ManagedThreadUtil::is_stopped(&mut stopped, &ctl_pair);
                            if !stopped {
                                // Check if service is stopping else panic
                                panic!("Could not send notification:{:?} ", err);
                            }
                        }
                    }
                }
                Err(err) => {
                    const ENODEV: i32 = 19;
                    const EPIPE: i32 = 32;
                    if err.raw_os_error() == Some(ENODEV) {
                        err_slot!(&path, "Notification device disconnected");
                        break;
                    }
                    if err.raw_os_error() == Some(EPIPE) {
                        debug!("Driver file closed, notification interrupted {}", path);
                        if stopped {
                            break;
                        }
                    }
                    else {
                        info_slot!(&path, format!("Notification file error: {}", err.to_string()));
                    }
                }
            }
            drop(fh);
        }
        info_slot!(&path, "Stopped notification handler")
    }
}