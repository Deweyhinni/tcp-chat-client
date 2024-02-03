// chat client
use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::collections::VecDeque;
use std::io::{Error, Read, Write};
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn start(address: Ipv4Addr, port: u16, buffer_capacity: usize) -> Result<Self, Error> {
        Ok(Self {
            // message_buffer: { Arc::new(Mutex::new(VecDeque::with_capacity(buffer_capacity))) },
            stream: { TcpStream::connect(SocketAddrV4::new(address, port))? },
        })
    }
}

fn main() {
    println!("Hello, world!");
}
