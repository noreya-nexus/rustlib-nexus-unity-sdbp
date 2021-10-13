pub const CLASS_ID : u8 = 0x01;

#[allow(dead_code)]
pub mod classes {

    pub mod transaction_error {
        pub const ID : u8 = 0x01;
    }

    pub mod descriptor {
        pub const ID : u8 = 0x02;

        pub mod operation_code {
            pub const ERROR : u8             = 0x01;
            pub const VENDOR_PRODUCT_ID : u8 = 0x02;
            pub const SERIAL_CODE : u8       = 0x03;
            pub const FW_VERSION : u8        = 0x04;
            pub const HW_VERSION : u8        = 0x05;
            pub const MAX_SLCK_SPEED : u8    = 0x06;
            pub const MAX_FRAME_SIZE : u8    = 0x07;
            pub const PROTOCOL_VERSION : u8  = 0x08;
            pub const VENDOR_NAME : u8       = 0x09;
            pub const PRODUCT_NAME : u8      = 0x0A;
            pub const BOOTLOADER_STATE : u8  = 0x0B;
            pub const MAX_POWER_3V3 : u8     = 0x0C;
            pub const MAX_POWER_5V : u8      = 0x0D;
            pub const MAX_POWER_12V : u8     = 0x0E;
        }
    }

    pub mod control {
        pub const ID : u8 = 0x03;

        pub mod operation_code {
            pub const RFU : u8               = 0x00;
            pub const ERROR : u8             = 0x01;
            pub const MODE_SUSPEND : u8      = 0x02;
            pub const MODE_RUN : u8          = 0x03;
            pub const MODE_BOOTLOADER : u8   = 0x04;
            pub const SYSTEM_RESET : u8      = 0x05;
            pub const FACTORY_RESET : u8     = 0x06;
            pub const SET_FRAME_SIZE : u8    = 0x07;
            pub const SET_SCLK_SPEED : u8    = 0x08;
            pub const UPDATE_DESCRIPTOR : u8 = 0x09;
        }
    }

    pub mod dummy {
        pub const ID : u8 = 0x04;

        pub mod operation_code {
            pub const RFU : u8   = 0x00;
            pub const DUMMY : u8 = 0x01;
        }

    }

    pub mod wait {
        pub const ID : u8 = 0x05;
        pub mod operation_code {
            pub const RFU : u8   = 0x00;
            pub const ERROR : u8 = 0x01;
            pub const WAIT : u8  = 0x02;
        }
    }

    pub mod notification {
        pub const ID : u8 = 0x06;

        pub mod operation_code {
            pub const RFU : u8               = 0x00;
            pub const ERROR : u8             = 0x01;
            pub const GET_NOTIFICATION : u8  = 0x02;
        }
    }

}


