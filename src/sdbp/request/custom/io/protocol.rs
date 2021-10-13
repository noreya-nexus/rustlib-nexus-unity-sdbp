pub const CLASS_ID : u8 = 0x03;

#[allow(dead_code)]
pub mod classes {

    pub mod input_class {

        pub const ID : u8 = 0x01;

        pub mod operation_code {
            pub const RFU: u8 = 0x00;
            pub const ERROR: u8 = 0x01;
            pub const SET_INPUT_MODE: u8 = 0x02;
            pub const SET_ANALOG_THRESHOLD: u8 = 0x03;
            pub const SET_DIGITAL_INTERRUPT: u8 = 0x04;
            pub const SET_DIGITAL_COUNTER: u8 = 0x05;
            pub const GET_VALUES: u8 = 0x06;
            pub const GET_CURRENT_VALUES: u8 = 0x07;
        }

        pub mod error_code {
            pub const OK : u8 = 0x00;
            pub const COMMAND_INVALID : u8 = 0x01;
            pub const WRONG_LENGTH : u8 = 0x02;
            pub const INVALID_MODE : u8 = 0x03;
            pub const PIN_OUT_OF_RANGE : u8 = 0x04;
            pub const INVALID_DIRECTION : u8 = 0x05;
            pub const INVALID_VALUE : u8 = 0x06;
        }
    }

    pub mod output_class {

        pub const ID : u8 = 0x02;

        pub mod operation_code {
            pub const RFU: u8 = 0x00;
            pub const ERROR: u8 = 0x01;
            pub const SET_OUTPUT: u8 = 0x02;
        }

        pub mod error_code {
            pub const OK : u8 = 0x00;
            pub const COMMAND_INVALID : u8 = 0x01;
            pub const WRONG_LENGTH : u8 = 0x02;
            pub const INVALID_MODE : u8 = 0x03;
            pub const PIN_OUT_OF_RANGE : u8 = 0x04;
            pub const INVALID_VALUE : u8 = 0x05;
            pub const POWER_CONFIG_MISSING : u8 = 0x06;
            pub const EXTERNAL_VOLTAGE : u8 = 0x07;
        }
    }

    pub mod power_management_class {

        pub const ID : u8 = 0x03;

        pub mod operation_code {
            pub const RFU: u8 = 0x00;
            pub const ERROR: u8 = 0x01;
            pub const SET_POWER_CONFIG : u8 = 0x02;
            pub const TEST_POWER_CONFIG : u8 = 0x03;
        }

        pub mod error_code {
            pub const OK : u8 = 0x00;
            pub const COMMAND_INVALID : u8 = 0x01;
            pub const WRONG_LENGTH : u8 = 0x02;
            pub const INVALID_RAIL : u8 = 0x03;
            pub const INVALID_VALUE : u8 = 0x04;
            pub const INVALID_MODE : u8 = 0x05;
        }
    }
}