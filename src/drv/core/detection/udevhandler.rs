use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;
use udev::{Device, Event, EventType};
use crate::drv::core::{DeviceEvent, DeviceEventBuilder, DeviceEventType, DeviceFilter, DeviceHandler};
use crossbeam_channel::{Sender};
use crate::datatypes::BootloaderState;

pub const DEVICE_CLASS: &str = "sdbp";

pub fn init_devices(sender : &Sender<DeviceEvent>, filter: &DeviceFilter<String>) {
    let mut enumerator = udev::Enumerator::new().unwrap();
    enumerator.match_subsystem(DEVICE_CLASS).unwrap();

    for device in enumerator.scan_devices().unwrap() {
        if is_compatible(&device, filter) {
            let path_device = PathBuf::from_str(device.syspath().to_string_lossy().to_string().as_str()).unwrap();
            let nr = DeviceHandler::get_device_nr(&path_device);
            sender.send(DeviceEventBuilder::generate(DeviceEventType::Connected, nr, &path_device,false)).expect("Could not send connected event");
        }
    }
}

fn is_compatible(device: &Device, filter: &DeviceFilter<String>) -> bool {
    let vendor_product_id = device.attribute_value("vendor_product_id").unwrap_or("".as_ref()).to_string_lossy().to_string();
    let bootloader_state = device.attribute_value("bootloader_state").unwrap_or("".as_ref()).to_string_lossy().to_string();
    if BootloaderState::try_from(bootloader_state.as_str()).unwrap_or(BootloaderState::BootloaderMode) == BootloaderState::BootloaderMode {
        return false; // Ignore devices in bootloader mode
    }
    if filter.is_match(vendor_product_id) {
        return true;
    }
    return false;
}

fn get_device(event: Event) -> (String, u16, PathBuf) {
    let device = event.device().syspath().to_string_lossy().to_string();
    let path_device = PathBuf::from_str(device.as_str()).unwrap();
    let nr = DeviceHandler::get_device_nr(&path_device);
    return (device, nr, path_device)
}

pub fn event_happened(sender : &Sender<DeviceEvent>, event: Event, filter: &DeviceFilter<String>) {
    debug!("Signal {:?}", event.device());

    if event.event_type() == EventType::Unbind {
        // Remove does not have attributes
        let (device, nr, path_device) = get_device(event);
        info!("Removed Device: {:?}", &device);
        sender.send(DeviceEventBuilder::generate(DeviceEventType::Disconnected, nr, &path_device, false)).expect("Could not send disconnected event");
        return;
    }

    if is_compatible(&event.device(), filter) {
        if event.event_type() == EventType::Bind {
            let (device, nr, path_device) = get_device(event);
            info!("Connected Device: {:?}", &device);
            sender.send(DeviceEventBuilder::generate(DeviceEventType::Connected, nr, &path_device, false)).expect("Could not send connected event");
        }  else if event.event_type() == EventType::Change {
            let (device, nr, path_device) = get_device(event);
            debug!("Updated Device: {:?}", &device);
            sender.send(DeviceEventBuilder::generate(DeviceEventType::Updated, nr, &path_device, false)).expect("Could not send update event");
        }
    }
}
