use crate::sdbp::{StandardBuilder, CustomBuilder,CoreBuilder};

pub struct FrameBuilder{}

impl FrameBuilder {

    pub fn new() -> FrameBuilder {
        FrameBuilder {}
    }

    pub fn core(self) -> CoreBuilder { CoreBuilder::new()  }
    pub fn standard(self) -> StandardBuilder {
        StandardBuilder::new()
    }
    pub fn custom(self) {   CustomBuilder{};  }
}