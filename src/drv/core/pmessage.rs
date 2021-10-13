const VIRTUAL_DEVICE_MASK : u16 = 0x2000;
const UDS_CLIENT_MASK : u16 = 0x1000;

#[derive(Debug)]
pub struct PMsg {

    src: u16,
    dst: u16,
    message: Result<Vec<u8>, std::io::Error>,
}

impl PMsg {

    pub fn is_client(id: u16) -> bool {
        if (id & UDS_CLIENT_MASK) != 0 {
            return true;
        }
        return false;
    }

    pub fn is_device(id: u16) -> bool {
        if (id & VIRTUAL_DEVICE_MASK) != 0 && (id & UDS_CLIENT_MASK) != 0  {
            return true;
        }
        return false;
    }

    pub fn is_virtual_device(id: u16) -> bool {
        if (id & VIRTUAL_DEVICE_MASK) != 0 {
            return true;
        }
        return false;
    }



    pub fn create(src: u16, dst: u16, msg: Result<Vec<u8>,std::io::Error> ) -> PMsg {
        PMsg{src,dst, message: msg}
    }


    pub fn get_msg(&self) -> Option<Vec<u8>> {

        match &self.message {
            Err(_err) => return None,
            Ok(_value) => return  Some(_value.clone()),
        }
    }

    pub fn get_src(&self) -> u16{
        self.src
    }
    pub fn get_dst(&self) -> u16 {
        self.dst
    }
}

impl std::fmt::Display for PMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(vsrc: {}, dst: {}, msg: {:?})", self.src, self.dst, self.message)
    }
}


