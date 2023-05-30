use std::{env, fs, process};
use std::convert::TryInto;
use std::io::{ErrorKind};
use std::path::{PathBuf};
use std::str::FromStr;
use std::time::Duration;
use std::io::Error;

use rand::Rng;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;

use crate::datatypes::Descriptor;
use crate::drv::core::{DeviceHandle, PMsg};
use crate::sdbp::{CoreBuilder, FrameBuilder, request};
use crate::util::{ChannelPair, ManagedThreadState, ManagedThreadUtil, spawn};

use super::notification_handler::*;

#[macro_export]
macro_rules! info_slot{
    ($a:expr,$b:expr)=>{
        {
            info!("slot {}: {}", $a, $b);
        }
    }
}

#[macro_export]
macro_rules! warn_slot{
    ($a:expr,$b:expr)=>{
        {
            warn!("slot {}: {}", $a, $b);
        }
    }
}

#[macro_export]
macro_rules! err_slot{
    ($a:expr,$b:expr)=>{
        {
            error!("slot {}: {}", $a, $b);
        }
    }
}

pub struct SdbpModule {}

const NO_NOTIFICATION_PENDING: [u8; 4] = [request::core::protocol::CLASS_ID, request::core::protocol::classes::notification::ID, request::core::protocol::classes::notification::operation_code::ERROR, 0x03];

impl SdbpModule {
    fn transfer(dev_handle: &mut DeviceHandle, buf: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
        let res = dev_handle.write(buf);
        if res.is_err() {
            return Err(res.unwrap_err());
        }

        let mut response = vec![0; 4096];
        let res = dev_handle.read(&mut response);
        match res {
            Ok(value) => return Ok(Vec::from(&response[0..value])),
            Err(_err) => return Err(_err),
        };
    }

    fn is_connected(device_path: &String) -> bool {
        let path = PathBuf::from(device_path);
        let mut cnt = 0;
        while cnt < 200 {
            if !path.as_path().exists() {
                return false;
            }
            cnt += 1;
            std::thread::sleep(Duration::from_millis(1));
        }
        return true;
    }

    fn is_not_get_notification(raw: &[u8]) -> bool {
        if raw[0] == request::core::protocol::CLASS_ID &&
            raw[1] == request::core::protocol::classes::notification::ID &&
            raw[2] == request::core::protocol::classes::notification::operation_code::GET_NOTIFICATION {
            return false;
        }
        return true;
    }

    fn is_suspend(raw: &[u8]) -> bool {
        if raw[0] == request::core::protocol::CLASS_ID &&
            raw[1] == request::core::protocol::classes::control::ID &&
            raw[2] == request::core::protocol::classes::control::operation_code::MODE_SUSPEND {
            return true;
        }
        return false;
    }

    fn stop(device_path: &String, ctl_chn: &ChannelPair<ManagedThreadState>,  timeout: Duration) -> Result<(), std::io::Error> {
        let result = ctl_chn.tx().send(ManagedThreadState::STOPPED);

        match result {
            Err(err) => {
                trace!("{:?}", err);
                trace!("Thread already stopped or channel is inactive");
                return Ok(());
            }
            _ => (),
        }

        match fs::write(format!("{}/close_notification", device_path), &[1,]) {
            Ok(_) => {}
            Err(err) => {
                err_slot!(&device_path, format!("Close notification error: {}", err));
            }
        }

        let input = ctl_chn.rx().recv_timeout(timeout).unwrap_or(ManagedThreadState::UNDEFINED);

        match input {
            ManagedThreadState::OK => {
                return Ok(());
            },
            ManagedThreadState::UNDEFINED => {
                // Note: This is ok because some threads may already be dead
            },
            _ => (),
        };
        return Err(Error::new(std::io::ErrorKind::TimedOut, format!("Cannot stop thread")));
    }

    pub fn handle_function(desc: Descriptor, ctl_pair: ChannelPair<ManagedThreadState>, dev_pair: ChannelPair<PMsg>, compatible_fw_major: u16, compatible_fw_minor: u16 ) {
        let mut stopped = false;
        let mut err_cnt: u32 = 0;
        let thread_name = std::thread::current().name().expect("Could not get thread name").to_string();
        let path = desc.path().to_str().expect("Could not get path").to_string();
        debug!("Started {} for {}" , &thread_name, &path);


        let tmp = desc.clone();
        let (notification_chn, notification_sender) = ChannelPair::new_bound();
        let notification_handler = spawn("NotifHandler".to_string(), |inner_ctl_pair| NotificationHandler::task(tmp, inner_ctl_pair, notification_sender));


        let mut latest_notification: Option<Vec<u8>> = None;
        let mut open_file_errors: u32 = 0;
        while !stopped {
            ManagedThreadUtil::is_stopped(&mut stopped, &ctl_pair);
            info!("Started driver for {}" , &path);

            //Init Sequence
            let result = DeviceHandle::new(&desc.dev_file());
            let mut dev_handle = match result {
                None => {
                    debug!("{:?} - Cannot open device file", desc.dev_file());
                    std::thread::sleep(Duration::from_millis(500)); // WARNING: This affects the connection time!
                    open_file_errors += 1;
                    if open_file_errors == 120  {
                        error!("Could not open {:?} after {} tries", desc.dev_file(), open_file_errors);
                        stopped = true;
                    }
                    continue;
                }
                Some(value) => value,
            };

            match SdbpModule::transfer(&mut dev_handle, CoreBuilder::new().descriptor().fw_version().unwrap()) {
                Ok(response) => {
                    if response[0] == 0x01 && response[1] == 0x02 && response[2] == 0x04 && response.len() == 10 {
                        let stability = response[3];  // Note: stability is ignored
                        let stability = match stability {
                            1 => "A",
                            2 => "B",
                            3 => "S",
                            _ => panic!("Could not check fw version stability flag")
                        };
                        let major = ((response[4] as u16) << 8) | response[5] as u16;
                        let minor = ((response[6] as u16) << 8) | response[7] as u16;
                        let patch = ((response[8] as u16) << 8) | response[9] as u16;
                        info!("Firmware version: {}.{}.{}.{}", stability, major, minor, patch);
                        if major != compatible_fw_major {
                            error!("Firmware version (major) not compatible");
                            stopped = true;
                        }
                        if minor < compatible_fw_minor {
                            error!("Firmware version (minor) not compatible");
                            stopped = true;
                        }
                    } else {
                        error!("Firmware version check response invalid");
                        stopped = true;
                    }
                }
                Err(_) => {
                    error!("Firmware version check failed");
                    stopped = true;
                }
            };

            let max_speed = env::var("MAX_SCLK_SPEED_KHZ");
            let speed_setting = match max_speed {
                Ok(max_env_speed) => {
                    let max_speed: u32 = u32::from_str(max_env_speed.as_str()).expect("MAX_SCLK_SPEED_KHZ value invalid");

                    if max_speed < 100 || max_speed > desc.max_sclk_speed() { // in kHz
                    error!("MAX_SCLK_SPEED_KHZ value out of range");
                        stopped = true;
                    }
                    info!("Limiting speed to {}kHz", max_speed);
                    max_speed
                }
                Err(_) => {
                    desc.max_sclk_speed()
                }
            };

            info!("Setting communication speed to: {} kHz", speed_setting);
            match SdbpModule::transfer(&mut dev_handle, CoreBuilder::new().control().set_sclk_speed(speed_setting).unwrap()) {
                Ok(response) => {
                    if response[0] != 0x01 || response[1] != 0x03 || response[2] != 0x08 || response[3] != 0x00 {
                        error!("Communication speed change failed");
                        stopped = true;
                    }
                }
                Err(_) => {
                    error!("Failed setting communication speed");
                    stopped = true;
                }
            };

            if stopped {
                // Terminate using os signal
                signal::kill(Pid::from_raw(process::id().try_into().expect("Could not get own process id")), Signal::SIGTERM).expect("Could not send TERM signal");
            }

            while !stopped {
                ManagedThreadUtil::is_stopped(&mut stopped, &ctl_pair);
                // Randomize timeout value to avoid all devices sending synchronous which may cause a blocked bus
                let random_timeout = rand::thread_rng().gen_range(0..20);

                let com_result = &dev_pair.rx().recv_timeout(Duration::from_millis(90 + random_timeout));
                let mut reset_after_suspend = false;
                match com_result {
                    Ok(msg) => {
                        trace!("{:?} - rx - {:?}",&path,msg);
                        match msg.get_msg() {
                            None => { warn!("Received message is empty"); }
                            Some(command) => {
                                if SdbpModule::is_suspend(command.as_slice()) {
                                    reset_after_suspend = true;
                                }

                                if SdbpModule::is_not_get_notification(command.as_slice()) {
                                    let mut response = Err(std::io::Error::from(ErrorKind::NotConnected));
                                    for i in 0..3 {
                                        // Try to send it three times before failure
                                        let ret = SdbpModule::transfer(&mut dev_handle, command.clone());

                                        match &ret {
                                            Ok(_) => {
                                                response = ret;
                                                break;
                                            }
                                            Err(err) => {
                                                if err.kind() == ErrorKind::NotConnected {
                                                    info_slot!(&path, "Device disconnected");
                                                    stopped=true;
                                                    break;
                                                }
                                                warn_slot!(&path, format!("Could not send message to device (attempt {}), retrying", i));
                                                err_cnt += 1;
                                            }
                                        }
                                        if i == 2 {
                                            warn_slot!(&path, "Return error");
                                            response = ret; // Return error if it fails x times
                                        }
                                    }
                                    trace!("{:?} - tx - {:?}",&path,msg);
                                    let answer = PMsg::create(msg.get_dst(), msg.get_src(), response);
                                    debug!("Answer: {:?}", answer);
                                    match dev_pair.tx().send(answer) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            // Client is gone
                                            info_slot!(&path, "Could not send message to client");
                                        }
                                    };
                                } else {
                                    let answer = match &latest_notification {
                                        None => PMsg::create(msg.get_dst(), msg.get_src(), Ok(Vec::from(NO_NOTIFICATION_PENDING))),
                                        Some(value) => {
                                            PMsg::create(msg.get_dst(), msg.get_src(), Ok(value.clone()))
                                        }
                                    };
                                    trace!("{:?} - tx - {:?}", &path, msg);
                                    match dev_pair.tx().send(answer) {
                                        Ok(_) => {
                                            latest_notification = None;
                                        }
                                        Err(_) => {
                                            // Client is gone
                                            info_slot!(&path, "Could not send notification");
                                        }
                                    };
                                }
                            }
                        };

                    }
                    Err(_) => {} // Timeout
                };

                if latest_notification.is_none() { // Receive the next notification only if the old one is reset
                    let result = notification_chn.rx().recv_timeout(Duration::from_millis(10));
                    match result {
                        Ok(value) => {
                            match value.get_msg() {
                                None => {
                                    warn_slot!(&path, "Notification was empty");
                                }
                                Some(val) => {
                                    debug!("Received Notification {:?}", &val);
                                    latest_notification = Some(val);
                                }
                            };
                        }
                        Err(_) => () // No notification pending
                    }
                }
                if reset_after_suspend {
                    let _discard = notification_chn.rx().recv_timeout(Duration::from_millis(1)); // Discard notification in buffer
                    latest_notification = None;
                    match SdbpModule::transfer(&mut dev_handle, FrameBuilder::new().core().control().update_descriptor().unwrap()) {
                        Err(err) => {
                            if err.kind() == ErrorKind::NotConnected {
                                info_slot!(&path, "Device disconnected");
                                stopped=true;
                                break;
                            }
                            err_cnt += 1;
                            warn_slot!(&path, format!("Update descriptor failed {}", err_cnt));
                            // Warn but ignore failure
                        }
                        Ok(_) => (), // Update descriptor after suspend
                    };
                }

                const TRIES: u8 = 10;
                let mut send_cnt = 0;
                while send_cnt < TRIES { // Try to send it 3 times
                    match SdbpModule::transfer(&mut dev_handle, FrameBuilder::new().core().control().mode_run().unwrap()) {
                        Err(err) => {
                            if err.kind() == ErrorKind::NotConnected {
                                info_slot!(&path, "Device disconnected");
                                stopped=true;
                                break;
                            }
                            err_cnt += 1;
                            warn_slot!(&path, format!("Send MODE_RUN failed {}", err_cnt));
                            std::thread::sleep(Duration::from_millis(10));
                            send_cnt += 1;
                        }
                        Ok(_) => break,
                    };
                }

                if send_cnt >= TRIES {
                    info_slot!(&path, format!("Communication failed {} times in a row, checking connection...", send_cnt));
                    if !SdbpModule::is_connected(&path) {
                        stopped = true;
                        err_slot!(&path, "Module disconnected");
                    }
                    info_slot!(&path, "Module is still connected");
                }

            }
            drop(dev_handle);
        }
        match Self::stop(&path,&notification_handler.chn, Duration::from_millis(200)) {
            Ok(_) => {}
            Err(err) => {
                trace!("Could not stop notification handler: {} ({})", err.to_string(),  &path)
            }
        };
        info_slot!(&path, "Stopped driver");
        debug!("Stopped {}", &thread_name);
    }
}