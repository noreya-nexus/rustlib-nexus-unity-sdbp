use crate::sdbp::request::core::control::ControlBuilder;
use crate::sdbp::request::core::descriptor::DescriptorBuilder;
use crate::sdbp::request::core::notification::NotificationBuilder;

pub struct CoreBuilder {}

impl CoreBuilder {
    pub fn new() -> CoreBuilder {
        CoreBuilder {}
    }
    pub fn control(self) -> ControlBuilder {
        ControlBuilder::new()
    }
    pub fn descriptor(self) -> DescriptorBuilder { DescriptorBuilder::new() }
    pub fn notification(self) -> NotificationBuilder { NotificationBuilder::new() }
}