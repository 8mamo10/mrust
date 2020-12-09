extern crate mio;
#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::io::{Read, Write};

use mio::tcp::{TcpListener, TcpStream};
use mio::*;

const SERVER: Token = Token(0);
const WEBROOT: &str = "/webroot";

struct WebServer {
    listening_socket: TcpListener,
    connections: HashMap<usize, TcpStream>,
    next_connection_id: usize,
}

impl WebServer {
    fn new(addr: &str) -> Result<Self, failure::Error> {
        let address = addr.parse()?;
        let listening_socket = TcpListener::bind(&address)?;
        Ok(WebServer {
            listening_socket,
            connections: HashMap::new(),
            next_connection_id: 1,
        })
    }
    fn run(&mut self) -> Result<(), failure::Error> {
        let poll = Poll::new()?;
        poll.register(
            &self.listening_socket,
            SERVER,
            Ready::readable(),
            PollOpt::level(),
        )?;
        let mut events = Events::with_capacity(1024);
        let mut response = Vec::new();
        loop {
            match poll.poll(&mut events, None) {
                Ok(_) => {}
                Err(e) => {
                    error!("{}", e);
                    continue;
                }
            }
            for event in &events {
                match event.token() {
                    SERVER => {
                        let (stream, remote) = match self.listening_socket.accept() {
                            Ok(t) => t,
                            Err(e) => {
                                error!("{}", e);
                                continue;
                            }
                        };
                        debug!("Connection from {}", &remote);
                        self.register_connection(&poll, stream)
                            .unwrap_or_else(|e| error!("{}", e));
                    }
                    Token(conn_id) => {
                        self.http_handler(conn_id, event, &poll, &mut response)
                            .unwrap_or_else(|e| error!("{}", e));
                    }
                }
            }
        }
    }
    fn register_connection(
        &mut self,
        poll: &Poll,
        stream: TcpStream,
    ) -> Result<(), failure::Error> {
        let token = Token(self.next_connection_id);
        poll.register(&stream, token, Ready::readable(), PollOpt::edge())?;
        if self
            .connections
            .insert(self.next_connection_id, stream)
            .is_some()
        {
            error!("Connection ID is already in use.");
        }
        self.next_connection_id += 1;
        Ok(())
    }
    fn http_handler(
        &mut self,
        conn_id: usize,
        event: Event,
        poll: &Poll,
        response: &mut Vec<u8>,
    ) -> Result<(), failure::Error> {
        let stream = self
            .connections
            .get_mut(&conn_id)
            .ok_or_else(|| failure::err_msg("Failed to get connection."))?;
        if event.readiness().is_readable() {
            debug!("readable conn_id: {}", conn_id);
            let mut buffer = [0u8; 1024];
            let nbytes = stream.read(&mut buffer)?;
            if nbytes != 0 {
                *response = WebServer::make_response(&buffer[..nbytes])?;
                poll.reregister(stream, Token(conn_id), Ready::writable(), PollOpt::edge())?;
            } else {
                self.connections.remove(&conn_id);
            }
            Ok(())
        } else if event.readiness().is_writable() {
            debug!("writable conn_id: {}", conn_id);
            stream.write_all(response)?;
            self.connections.remove(&conn_id);
            Ok(())
        } else {
            Err(failure::err_msg("Undefined event."))
        }
    }
    fn make_response(buffer: &[u8]) -> Result<Vec<u8>, failure::Error> {
        // tmp
        WebServer::create_msg_from_code(501, None)
    }

    fn create_msg_from_code(
        status_code: u16,
        msg: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, failure::Error> {
        // tmp
        Ok("Test".to_string().into_bytes())
    }
}

fn main() {
    println!("Hello, world!");
}
