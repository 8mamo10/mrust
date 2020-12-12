const OP: usize = 0;

const DHCP_MINIMUM_SIZE: usize = 237;
const OPTION_END: u8 = 255;

pub struct DhcpPacket {
    buffer: Vec<u8>,
}

impl DhcpPacket {
    //pub fn new(buf: Vec<u8>) -> Result<DhcpPacket, failure::Error> {
    pub fn new(buf: Vec<u8>) -> Option<DhcpPacket> {
        if buf.len() > DHCP_MINIMUM_SIZE {
            let packet = DhcpPacket { buffer: buf };
            return Some(packet);
        }
        None
    }
    pub fn get_op(&self) -> u8 {
        self.buffer[OP]
    }
}

pub struct DhcpServer {}

impl DhcpServer {
    pub fn new() -> Result<DhcpServer, failure::Error> {
        Ok(DhcpServer {})
    }
}
