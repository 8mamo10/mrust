extern crate rayon;

use pnet::packet::{ip, tcp};
use pnet::transport::{self, TransportProtocol};
use std::{collections, env, fs, net, thread, time};

#[macro_use]
extern crate log;

const TCP_SIZE: usize = 20;
const MAXIMUM_PORT_NUM: u16 = 1023;

#[derive(Debug)]
struct PacketInfo {
    my_ipaddr: net::Ipv4Addr,
    target_ipaddr: net::Ipv4Addr,
    my_port: u16,
    scan_type: ScanType,
}

#[derive(Copy, Clone, Debug)]
enum ScanType {
    SynScan = tcp::TcpFlags::SYN as isize,
    FinScan = tcp::TcpFlags::FIN as isize,
    XmasScan = (tcp::TcpFlags::FIN | tcp::TcpFlags::URG | tcp::TcpFlags::PSH) as isize,
    NullScan = 0,
}

fn main() {
    // env::set_var("RUST_LOG", "debug");
    // env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        error!("Bad number of arguments. [ipaddr][scantype]");
        std::process::exit(1);
    }
    let packet_info = {
        let contents = fs::read_to_string(".env").expect("Failed to read env file");
        let lines: Vec<_> = contents.split('\n').collect();
        let mut map = collections::HashMap::new();
        for line in lines {
            let elm: Vec<_> = line.split('=').map(str::trim).collect();
            if elm.len() == 2 {
                map.insert(elm[0], elm[1]);
            }
        }
        PacketInfo {
            my_ipaddr: map["MY_IPADDR"].parse().expect("invalid ipaddr"),
            target_ipaddr: args[1].parse().expect("invalid target ipaddr"),
            my_port: map["MY_PORT"].parse().expect("invalid port number"),
            scan_type: match args[2].as_str() {
                "sS" => ScanType::SynScan,
                "sF" => ScanType::FinScan,
                "sX" => ScanType::XmasScan,
                "sN" => ScanType::NullScan,
                _ => {
                    error!("Undefined scan method, only accept [sF|sF|sX|sN].");
                    std::process::exit(1);
                }
            },
        }
    };
    println!("PacketInfo: {:?}", packet_info);

    let (mut ts, mut tr) = transport::transport_channel(
        1024,
        transport::TransportChannelType::Layer4(TransportProtocol::Ipv4(
            ip::IpNextHeaderProtocols::Tcp,
        )),
    )
    .unwrap();

    rayon::join(
        || send_packet(&mut ts, &packet_info),
        || receive_packets(&mut tr, &packet_info),
    );
}

fn send_packet(ts: &mut transport::TransportSender, packet_info: &PacketInfo) {
    let mut packet = build_packet(packet_info);
    for i in 1..MAXIMUM_PORT_NUM + 1 {
        let mut tcp_header = tcp::MutableTcpPacket::new(&mut packet).unwrap();
        register_destination_port(i, &mut tcp_header, packet_info);
        thread::sleep(time::Duration::from_millis(5));
        ts.send_to(tcp_header, net::IpAddr::V4(packet_info.target_ipaddr))
            .expect("failed to send");
    }
}

fn register_destination_port(
    target: u16,
    tcp_header: &mut tcp::MutableTcpPacket,
    packet_info: &PacketInfo,
) {
    tcp_header.set_destination(target);
    let checksum = tcp::ipv4_checksum(
        &tcp_header.to_immutable(),
        &packet_info.my_ipaddr,
        &packet_info.target_ipaddr,
    );
    tcp_header.set_checksum(checksum);
}

fn receive_packets(tr: &mut transport::TransportReceiver, packet_info: &PacketInfo) {
    let mut reply_ports = Vec::new();
    let mut packet_iter = transport::tcp_packet_iter(tr);
    loop {
        println!("receiving...");
        let tcp_packet = match packet_iter.next() {
            Ok((tcp_packet, _)) => {
                println!("Ok: receiving...");
                if tcp_packet.get_destination() != packet_info.my_port {
                    continue;
                }
                tcp_packet
            }
            Err(_) => {
                println!("Err: receiving...");
                continue;
            }
        };

        let target_port = tcp_packet.get_source();
        match packet_info.scan_type {
            ScanType::SynScan => {
                if tcp_packet.get_flags() == tcp::TcpFlags::SYN | tcp::TcpFlags::ACK {
                    println!("port {} is open", target_port);
                }
            }
            ScanType::FinScan | ScanType::XmasScan | ScanType::NullScan => {
                reply_ports.push(target_port);
            }
        }
        if target_port != MAXIMUM_PORT_NUM {
            continue;
        }
        match packet_info.scan_type {
            ScanType::FinScan | ScanType::XmasScan | ScanType::NullScan => {
                for i in 1..MAXIMUM_PORT_NUM + 1 {
                    match reply_ports.iter().find(|&&x| x == i) {
                        None => {
                            println!("port {} is open", i);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        return;
    }
}

fn build_packet(packet_info: &PacketInfo) -> [u8; TCP_SIZE] {
    let mut tcp_buffer = [0u8; TCP_SIZE];
    let mut tcp_header = tcp::MutableTcpPacket::new(&mut tcp_buffer[..]).unwrap();
    tcp_header.set_source(packet_info.my_port);
    tcp_header.set_data_offset(5);
    tcp_header.set_flags(packet_info.scan_type as u16);
    let checksum = tcp::ipv4_checksum(
        &tcp_header.to_immutable(),
        &packet_info.my_ipaddr,
        &packet_info.target_ipaddr,
    );
    tcp_header.set_checksum(checksum);

    return tcp_buffer;
}
