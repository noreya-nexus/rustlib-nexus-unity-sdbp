pub struct DrvMeta {
    product : String,
    name : String,
    socket_path : String,
}

impl DrvMeta {

    pub fn new(product : String, name : String, socket_path : String) -> DrvMeta {
        DrvMeta{product,name,socket_path}
    }

    pub fn product(&self) -> &String {
        return &self.product;
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn socket(&self) -> &String {
        return &self.socket_path;
    }

}