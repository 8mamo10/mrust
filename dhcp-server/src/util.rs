use std::collections::HashMap;
use std::io;
use std::net::{AddrParseError, IpAddr, Ipv4Addr, UdpSocket};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use pnet::packet::icmp::echo_request::{EchoRequestPacket, MutableEchoRequestPacket};
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pnet::transport::icmp_packet_iter;
use pnet::transport::{self, TransportChannelType, TransportProtocol::Ipv4};
use pnet::util::checksum;

pub fn is_ipaddr_available(target_ip: Ipv4Addr) -> Result<(), failure::Error> {
    let icmp_buf = create_default_icmp_buffer();

    let icmp_packet = EchoRequestPacket::new(&icmp_buf).unwrap();
    let (mut transport_sender, mut transport_receiver) = transport::transport_channel(
        1024,
        TransportChannelType::Layer4(Ipv4(IpNextHeaderProtocols::Icmp)),
    )?;
    transport_sender.send_to(icmp_packet, IpAddr::V4(target_ip))?;
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let mut itr = icmp_packet_iter(&mut transport_receiver);
        let (packet, _) = itr.next().unwrap();
        if packet.get_icmp_type() == IcmpTypes::EchoReply {
            match sender.send(true) {
                Err(_) => {
                    info!("icmp timeout!");
                }
                X => {
                    return;
                }
            }
        }
    });
    if receiver.recv_timeout(Duration::from_millis(200)).is_ok() {
        let message = format!("ip addr is already in use: {}", target_ip);
        warn!("{}", message);
        Err(failure::err_msg(message))
    } else {
        debug!("not received reply within timeout");
        Ok({})
    }
}

fn create_default_icmp_buffer() -> [u8; 8] {
    let mut buf = [0u8; 8];
    let mut icmp_packet = MutableEchoRequestPacket::new(&mut buf).unwrap();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    let checksum = checksum(icmp_packet.to_immutable().packet(), 16);
    icmp_packet.set_checksum(checksum);
    buf
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
