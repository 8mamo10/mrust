use pnet::packet::PrimitiveValues;
use pnet::util::MacAddr;
use std::net::Ipv4Addr;

const OP: usize = 0;

const XID: usize = 4;
const SECS: usize = 8;
const FLAGS: usize = 10;
const CIADDR: usize = 12;
const YIADDR: usize = 16;
const SIADDR: usize = 20;
const GIADDR: usize = 24;
const CHADDR: usize = 28;
const SNAME: usize = 44;

pub const OPTIONS: usize = 236;

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

    pub fn get_buffer(&self) -> &[u8] {
        self.buffer.as_ref()
    }

    pub fn get_op(&self) -> u8 {
        self.buffer[OP]
    }

    pub fn get_options(&self) -> &[u8] {
        &self.buffer[OPTIONS..]
    }

    pub fn set_giaddr(&mut self, giaddr: Ipv4Addr) {
        self.buffer[GIADDR..CHADDR].copy_from_slice(&giaddr.octets());
    }

    pub fn set_chaddr(&mut self, chaddr: MacAddr) {
        let t = chaddr.to_primitive_values();
        let macaddr_value = [t.0, t.1, t.2, t.3, t.4, t.5];
        self.buffer[CHADDR..CHADDR + 6].copy_from_slice(&macaddr_value);
    }
}

pub struct DhcpServer {}

impl DhcpServer {
    pub fn new() -> Result<DhcpServer, failure::Error> {
        Ok(DhcpServer {})
    }
}
