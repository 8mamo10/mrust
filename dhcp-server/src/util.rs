use std::collections::HashMap;
use std::io;
use std::net::{AddrParseError, Ipv4Addr, UdpSocket};

pub fn is_ipaddr_available(target_ip: Ipv4Addr) -> Result<(), failure::Error> {
    Ok({})
}

pub fn load_env() -> HashMap<String, String> {
    HashMap::new()
}
pub fn obtain_static_addresses(
    env: &HashMap<String, String>,
) -> Result<HashMap<String, Ipv4Addr>, AddrParseError> {
    Ok(HashMap::new())
}
pub fn make_big_endian_vec_from_u32(i: u32) -> Result<Vec<u8>, io::Error> {
    Ok(Vec::new())
}

pub fn send_dhcp_broadcast_response(soc: &UdpSocket, data: &[u8]) -> Result<(), failure::Error> {
    Ok({})
}
