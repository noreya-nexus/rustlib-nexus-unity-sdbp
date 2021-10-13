use crate::sdbp::request::custom::power::temperature::TemperatureBuilder;
use crate::sdbp::request::custom::power::powercmd::PowerBuilder;

mod powercmd;
mod temperature;
pub mod protocol;

pub struct Power{}

impl Power {

    pub fn new() -> Self{
        Power{}
    }

    pub fn power_builder() -> PowerBuilder {
        PowerBuilder::new()
    }

    pub fn temperature_builder() -> TemperatureBuilder {
        TemperatureBuilder::new()
    }

}