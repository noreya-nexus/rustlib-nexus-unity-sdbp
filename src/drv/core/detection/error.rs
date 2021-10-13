#[derive(Debug, Clone, PartialEq)]
pub enum ParseDescriptorErrorSource {
    BootloaderState,
    FwVersion,
    HwVersion,
    MaxFrameSize,
    MaxPower12v,
    MaxPower5v,
    MaxPower3v3,
    MaxSclkSpeed,
    ProductName,
    VendorName,
    VendorProductId,
    ProtocolVersion,
    Serial,
    Path,
    DevAdr,
    Rid,
    KernelDriver,
}
#[derive(Debug, Clone,PartialEq)]
pub struct ParseDescriptorError{
    source : ParseDescriptorErrorSource,
}

impl ParseDescriptorError{

    pub fn new(source : ParseDescriptorErrorSource) -> ParseDescriptorError {
        ParseDescriptorError {source}
    }
}