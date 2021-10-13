pub const CLASS_ID : u8 = 0x03;

#[allow(dead_code)]
pub mod classes {

    pub mod power_class {
        pub const ID : u8 = 0x01;

        pub mod operation_code {
            pub const RFU : u8                    = 0x00;
            pub const ERROR : u8                  = 0x01;
            pub const SOURCE : u8                 = 0x02;
            pub const CURRENT_LIMIT : u8          = 0x03;
            pub const VOLTAGE_CURRENT_STATUS: u8  = 0x04;
            pub const OPP_CNT_STATUS : u8         = 0x05;
        }
    }

    pub mod temperature_control_class {
        pub const ID : u8 = 0x02;
        pub mod operation_code {
            pub const RFU : u8                    = 0x00;
            pub const ERROR : u8                  = 0x01;
            pub const TEMPERATURE_SENSOR : u8     = 0x02;
            pub const FAN_STATUS : u8             = 0x03;
            pub const FAN_CONTROL: u8             = 0x04;
            pub const FAN_RPM : u8                = 0x05;
            pub const FAN_RPM_CONTROL : u8        = 0x06;

        }
    }


}