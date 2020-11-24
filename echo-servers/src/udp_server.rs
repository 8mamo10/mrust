use std::net::UdpSocket;
use std::str;

pub fn serve(address: &str) -> Result<(), failure::Error> {
    let server_socket = UdpSocket::bind(address).expect("Failed to bind socket");
    loop {
        let mut buf = [0u8; 1024];
        // let (size, src) = server_socket.recv_from(&mut buf);
        // debug!("Handling data from {}", src);
        // print!("{}", str::from_utf8(&buf[..size])?);
        // server_socket.send_to(&buf.src)?;

        match server_socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                println!("Handling data from{}", src);
                print!("{}", str::from_utf8(&buf[..size]).expect("Failed to read"));
                server_socket
                    .send_to(&buf[..size], src)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("could not receive a datagram: {}", e);
            }
        }
    }
}
