use crate::sdbp::request::custom::bmc::buzzer::BuzzerBuilder;
use crate::sdbp::request::custom::bmc::cmc::CmcBuilder;
use crate::sdbp::request::custom::bmc::usbhub::UsbHubBuilder;
use crate::sdbp::request::custom::bmc::voltage::VoltageBuilder;
use crate::sdbp::request::custom::bmc::watchdog::WatchdogBuilder;

pub mod voltage;
pub mod protocol;
pub mod buzzer;
pub mod watchdog;
pub mod cmc;
pub mod usbhub;

pub struct CustomBuilderBmc {}


impl CustomBuilderBmc {

    pub fn new() -> CustomBuilderBmc{
        CustomBuilderBmc{}
    }

    pub fn voltage() -> VoltageBuilder {
        VoltageBuilder::new()
    }

    pub fn buzzer() -> BuzzerBuilder {
        BuzzerBuilder::new()
    }

    pub fn watchdog() -> WatchdogBuilder {
        WatchdogBuilder::new()
    }

    pub fn cmc() -> CmcBuilder {
        CmcBuilder::new()
    }

    pub fn usbhub() -> UsbHubBuilder {
        UsbHubBuilder::new()
    }

}
