pub const CLASS_ID: u8 = 0x03;

#[allow(dead_code)]
pub mod classes {
    pub mod input {
        pub const ID: u8 = 0x01;

        pub mod operation_code {
            pub const RFU: u8 = 0x00;
            pub const ERROR: u8 = 0x01;
            pub const GET_VOLTAGE: u8 = 0x02;

            pub mod input {
                pub const RFU: u8 = 0x00;
                pub const NTC_0: u8 = 0x01;
                pub const RAIL_1V8: u8 = 0x02;
                pub const RTC_BAT: u8 = 0x03;
                pub const NTC_1: u8 = 0x04;
            }
        }
    }

    pub mod buzzer {
        pub const ID: u8 = 0x05;

        pub mod operation_code {
            pub const RFU: u8 = 0x00;
            pub const ERROR: u8 = 0x01;
            pub const MODE_BUZZER: u8 = 0x02;
        }
    }

    pub mod watchdog {
        pub const ID: u8 = 0x03;

        pub mod operation_code {
            pub const RFU: u8 = 0x00;
            pub const ERROR: u8 = 0x01;
            pub const ENABLE_TIMEOUT: u8 = 0x02;
            pub const DISABLE_TIMEOUT: u8 = 0x03;
            pub const GET_TIMEOUT: u8 = 0x04;
            pub const GET_TIME_LEFT: u8 = 0x05;
            pub const ALIVE: u8 = 0x06;
            pub const SAVE_CONFIG: u8 = 0x07;
            pub const SET_SHUTDOWN_TIMEOUT: u8 = 0x08;
            pub const GET_SHUTDOWN_TIMEOUT: u8 = 0x09;
            pub const SW_SHUTDOWN: u8 = 0x0A;
            pub const EMERGENCY_MODE_STATE: u8 = 0x0B;
        }
    }

    pub mod cmc {
        pub const ID: u8 = 0x04;

        pub mod operation_code {
            pub const RFU: u8 = 0x00;
            pub const ERROR: u8 = 0x01;
            pub const CTL_USBBOOT: u8 = 0x02;
        }
    }

    pub mod usbhub {
        pub const ID: u8 = 0x02;

        pub mod operation_code {
            pub const RFU: u8 = 0x00;
            pub const ERROR: u8 = 0x01;
            pub const GET_HUB_STATE: u8 = 0x02;
            pub const SET_HUB_STATE: u8 = 0x03;
            pub const GET_PORT_STATE: u8 = 0x04;
            pub const SET_PORT_STATE: u8 = 0x05;
            pub const HUB_RESET: u8 = 0x06;
        }

    }

    pub mod control {
        pub const CLASS_ID: u8 = 0x01;
        pub const ID: u8 = 0x03;

        pub mod operation_code {
            pub const RFU: u8 = 0x00;
            pub const ERROR: u8 = 0x01;
            pub const SYSTEM_RESET: u8 = 0x05;
        }

        pub mod return_code {
            pub const OK: (u8, &'static str) = (0x01, "success");
            pub const FAILED: (u8, &'static str) = (0x02, "failed");
        }
    }

    pub mod return_code {
        pub const OK: (u8, &'static str) = (0x00, "success");
        pub const FAILED: (u8, &'static str) = (0x01, "failed");
        pub const PROTECTED: (u8, &'static str) = (0x02, "flash is protected");
        pub const OUT_OF_RANGE: (u8, &'static str) = (0x03, "timeout setting is out of range");
        pub const WRONG_STATE: (u8, &'static str) = (0x04, "wrong state");
        pub const ERR_LIST: [(u8, &'static str); 4] = [FAILED, PROTECTED, OUT_OF_RANGE, WRONG_STATE];
    }
}