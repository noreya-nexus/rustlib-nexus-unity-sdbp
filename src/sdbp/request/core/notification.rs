use super::protocol::*;

pub struct NotificationBuilder{

    frame : Vec<u8>,

}

impl NotificationBuilder {

    pub fn new() -> NotificationBuilder {
        let mut frame = vec![];
        frame.push(CLASS_ID); //Core
        frame.push(classes::notification::ID);
        NotificationBuilder{
            frame
        }
    }
    pub fn get_notification(mut self) -> Vec<u8> {
        self.frame.push(classes::notification::operation_code::GET_NOTIFICATION);
        return self.frame;
    }
}