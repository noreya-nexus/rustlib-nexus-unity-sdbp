use super::*;

pub const DRV_DEV_ADR : u16 = 0xFFFF;

#[derive(Debug)]
pub enum Command {

    Info = 0x0002,
    GetDeviceList  = 0x0003,
    GetDescriptor = 0x0004,
    GetNotification = 0x0005,

    Device = 0x010,

    Error = 0x1001,
    Response = 0x1002,
}

impl Command {

    const DRV_DEV_ID : u16= 0xFFFF;


    pub fn from_frame(frame : &Request) -> Option<Command>{

        let dev_id = frame.get_dev_id();

        let result = match dev_id {

            Command::DRV_DEV_ID=> {

                let result = match frame.get_op_id()  {

                    op_id if op_id == Command::Info as u16 => Some(Command::Info),
                    op_id if op_id == Command::GetDeviceList as u16 => Some(Command::GetDeviceList),
                    op_id if op_id == Command::GetNotification as u16 => Some(Command::GetNotification),
                    op_id if op_id == Command::GetDescriptor as u16 => Some(Command::GetDescriptor),
                    _ => None,
                };
                result
            },
            _ => {

                let result = match frame.get_op_id() {
                    op_id if op_id == Command::Device as u16 => Some(Command::Device),
                    _ => None,
                };
                result
            },
        };
        result
    }
}