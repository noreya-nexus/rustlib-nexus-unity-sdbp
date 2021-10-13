use super::protocol::*;
use std::io::Error;
use std::io::ErrorKind;

pub struct InputBuilder {
    frame: Vec<u8>,
}

impl InputBuilder {
    pub fn new() -> InputBuilder {
        let frame = vec![CLASS_ID, classes::input_class::ID];
        InputBuilder { frame }
    }

    pub fn set_input_mode(mut self, pin_nr: u8, mode: u8) -> Result<Vec<u8>, Error> {
        match mode {
            1 | 2 => (),
            _ => return Err(Error::new(ErrorKind::Other, "Invalid input mode")),
        }

        self.frame
            .push(classes::input_class::operation_code::SET_INPUT_MODE);
        self.frame.push(pin_nr);
        self.frame.push(mode);
        Ok(self.frame)
    }

    pub fn set_analog_threshold(
        mut self,
        pin_nr: u8,
        threshold_mv: u16,
        trigger: &String,
    ) -> Result<Vec<u8>, Error> {
        let direction = match trigger.as_str() {
            "disabled" => 0,
            "rising" => 1,
            "falling" => 2,
            _ => return Err(Error::new(ErrorKind::Other, "Invalid trigger")),
        };

        match threshold_mv {
            50..=25000 => (),
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Invalid threshold_mv value (50-25000)",
                ))
            }
        }

        self.frame
            .push(classes::input_class::operation_code::SET_ANALOG_THRESHOLD);
        self.frame.push(pin_nr);
        self.frame.push((threshold_mv >> 8) as u8);
        self.frame.push(threshold_mv as u8);
        self.frame.push(direction);
        Ok(self.frame)
    }

    pub fn set_digital_interrupt(
        mut self,
        pin_nr: u8,
        debounce_time_ms: u16,
        trigger: &String,
    ) -> Result<Vec<u8>, Error> {
        let trigger_direction = match trigger.as_str() {
            "disabled" => 0,
            "rising" => 1,
            "falling" => 2,
            "pulse" => 3,
            _ => return Err(Error::new(ErrorKind::Other, "Invalid trigger")),
        };

        match debounce_time_ms {
            0..=1000 => (),
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Invalid debunce_time_ms value (0-1000)",
                ))
            }
        }

        self.frame
            .push(classes::input_class::operation_code::SET_DIGITAL_INTERRUPT);
        self.frame.push(pin_nr);
        self.frame.push((debounce_time_ms >> 8) as u8);
        self.frame.push(debounce_time_ms as u8);
        self.frame.push(trigger_direction);
        Ok(self.frame)
    }

    pub fn set_digital_counter(mut self, pin_nr: u8, state: &String) -> Result<Vec<u8>, Error> {
        let counter_state = match state.as_str() {
            "disabled" => 0,
            "enabled" => 1,
            _ => return Err(Error::new(ErrorKind::Other, "Invalid state")),
        };

        self.frame
            .push(classes::input_class::operation_code::SET_DIGITAL_COUNTER);
        self.frame.push(pin_nr);
        self.frame.push(counter_state);
        Ok(self.frame)
    }

    pub fn get_values(mut self) -> Result<Vec<u8>, Error> {
        self.frame
            .push(classes::input_class::operation_code::GET_VALUES);
        Ok(self.frame)
    }

    pub fn get_current_values(mut self) -> Result<Vec<u8>, Error> {
        self.frame
            .push(classes::input_class::operation_code::GET_CURRENT_VALUES);
        Ok(self.frame)
    }
}
