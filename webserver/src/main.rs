use std::collections::HashMap;

use mio::tcp::{TcpListener, TcpStream};
use mio::*;

struct WebServer {
    listening_socket: TcpListener,
    connections: HashMap<usize, TcpStream>,
    next_connection_id: usize,
}

fn main() {
    println!("Hello, world!");
}
