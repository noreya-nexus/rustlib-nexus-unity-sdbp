use super::protocol::*;

pub struct DescriptorBuilder{frame : Vec<u8>}

impl DescriptorBuilder {

    pub fn new() -> DescriptorBuilder {

        let mut frame = vec![];
        frame.push(CLASS_ID);
        frame.push(classes::descriptor::ID);
        DescriptorBuilder {frame}
    }

    pub fn vendor_product_id(mut self) -> Result<Vec<u8>,()>{

        self.frame.push(classes::descriptor::operation_code::VENDOR_PRODUCT_ID);
        Ok(self.frame)
    }

    pub fn serial_code(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::SERIAL_CODE);
        Ok(self.frame)
    }

    pub fn fw_version(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::FW_VERSION);
        Ok(self.frame)
    }

    pub fn hw_version(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::HW_VERSION);
        Ok(self.frame)
    }

    pub fn max_sclk_speed(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::MAX_SLCK_SPEED);
        Ok(self.frame)    }

    pub fn max_frame_size(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::MAX_FRAME_SIZE);
        Ok(self.frame)    }

    pub fn protocol_version(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::PROTOCOL_VERSION);
        Ok(self.frame)    }


    pub fn vendor_name(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::VENDOR_NAME);
        Ok(self.frame)    }


    pub fn product_name(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::PRODUCT_NAME);
        Ok(self.frame)    }

    pub fn bootloader_state(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::BOOTLOADER_STATE);
        Ok(self.frame)    }

    pub fn max_power_3v3(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::MAX_POWER_3V3);
        Ok(self.frame)    }

    pub fn max_power_5v(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::MAX_POWER_5V);
        Ok(self.frame)    }

    pub fn max_power_12v(mut self) -> Result<Vec<u8>,()> {

        self.frame.push(classes::descriptor::operation_code::MAX_POWER_12V);
        Ok(self.frame)    }
}

#[allow(unused_imports,unused_macros)]
mod test {
   use super::*;

    macro_rules! assert_result {
        ($result:expr,$msg:expr) => {
            match $result {
                Ok(value) => value,
                Err(_) => panic!($msg),
            }
        };
    }

    #[test]
    fn test_desc_vendor_product_id() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::VENDOR_PRODUCT_ID];
        let result = DescriptorBuilder::new().vendor_product_id();
        assert_eq!(expected, assert_result!(result,""),"Failed to build VENDOR_PRODUCT_ID")
    }

    #[test]
    fn test_desc_vendor_serial_code() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::SERIAL_CODE];
        let result = DescriptorBuilder::new().serial_code();
        assert_eq!(expected, assert_result!(result,""),"Failed to build SERIAL_CODE")
    }

    #[test]
    fn test_desc_fw_version() {
        let expected = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::FW_VERSION];
        let result = DescriptorBuilder::new().fw_version();
        assert_eq!(expected, assert_result!(result,""),"Failed to build FW_VERSION")
    }

    #[test]
    fn test_desc_hw_version() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::HW_VERSION];
        let result = DescriptorBuilder::new().hw_version();
        assert_eq!(expected, assert_result!(result,""),"Failed to build HW_VERSION")
    }

    #[test]
    fn test_desc_max_sclk_speed() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::MAX_SLCK_SPEED];
        let result = DescriptorBuilder::new().max_sclk_speed();
        assert_eq!(expected, assert_result!(result,""),"Failed to build MAX_SLCK_SPEED")
    }

    #[test]
    fn test_desc_max_frame_size() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::MAX_FRAME_SIZE];
        let result = DescriptorBuilder::new().max_frame_size();
        assert_eq!(expected, assert_result!(result,""),"Failed to build MAX_FRAME_SIZE")
    }

    #[test]
    fn test_desc_protocol_version() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::PROTOCOL_VERSION];
        let result = DescriptorBuilder::new().protocol_version();
        assert_eq!(expected, assert_result!(result,""),"Failed to build PROTOCOL_VERSION")
    }

    #[test]
    fn test_desc_vendor_name() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::VENDOR_NAME];
        let result = DescriptorBuilder::new().vendor_name();
        assert_eq!(expected, assert_result!(result,""),"Failed to build VENDOR_NAME")
    }

    #[test]
    fn test_desc_product_name() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::PRODUCT_NAME];
        let result = DescriptorBuilder::new().product_name();
        assert_eq!(expected, assert_result!(result,""),"Failed to build PRODUCT_NAME")
    }

    #[test]
    fn test_desc_bootloader_state() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::BOOTLOADER_STATE];
        let result = DescriptorBuilder::new().bootloader_state();
        assert_eq!(expected, assert_result!(result,""),"Failed to build BOOTLOADER_STATE")
    }

    #[test]
    fn test_desc_max_power_3v3() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::MAX_POWER_3V3];
        let result = DescriptorBuilder::new().max_power_3v3();
        assert_eq!(expected, assert_result!(result,""),"Failed to build MAX_POWER_3V3")
    }

    #[test]
    fn test_desc_max_power_5v() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::MAX_POWER_5V];
        let result = DescriptorBuilder::new().max_power_5v();
        assert_eq!(expected, assert_result!(result,""),"Failed to build MAX_POWER_5V")
    }

    #[test]
    fn test_desc_max_power_12v() {
        let expected : Vec<u8> = vec![CLASS_ID, classes::descriptor::ID, classes::descriptor::operation_code::MAX_POWER_12V];
        let result = DescriptorBuilder::new().max_power_12v();
        assert_eq!(expected, assert_result!(result,""),"Failed to build MAX_POWER_12V")
    }
}