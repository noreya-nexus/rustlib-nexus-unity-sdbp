use crate::sdbp::request::custom::io::input::InputBuilder;
use crate::sdbp::request::custom::io::output::OutputBuilder;
use crate::sdbp::request::custom::io::powermgmt::PowerMgmtBuilder;

pub mod protocol;
mod input;
mod output;
mod powermgmt;

pub struct IoBuilder{}

impl IoBuilder {

    pub fn new() -> Self{
        IoBuilder{}
    }

    pub fn input(self) -> InputBuilder {
        InputBuilder::new()
    }

    pub fn output(self) -> OutputBuilder {
        OutputBuilder::new()
    }

    pub fn powermgmt(self) -> PowerMgmtBuilder {
        PowerMgmtBuilder::new()
    }
}