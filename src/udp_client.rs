use std::net::UdpSocket;
use std::{io, str};

pub fn communicate(address: &str) -> Result<(), failure::Error> {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("failed to bind socket");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // cannot send
        // { code: 40, kind: Other, message: "Message too long" }
        //let mut input = "a".to_string().repeat(9218);

        // can send
        //let mut input = "a".to_string().repeat(9217);
        socket
            .send_to(input.as_bytes(), address)
            .expect("failed to send data");

        let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer).expect("failed to receive");
        print!(
            "{}",
            str::from_utf8(&buffer).expect("failed to convert to String")
        );
    }
}
