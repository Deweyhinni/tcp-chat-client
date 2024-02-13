// chat client
use std::net::{TcpStream, SocketAddrV4, Ipv4Addr};
// use std::collections::VecDeque;
use std::io::{Error, Read, Write};
use std::io;
use std::thread;
// use std::sync::Mutex;
// use std::sync::Arc;
// use std::thread;

fn split_u16(short_u16: u16) -> [u8;2] {
    let high_byte: u8 = (short_u16 >> 8) as u8;
    let low_byte: u8 = (short_u16 & 0xff) as u8;

    return [high_byte, low_byte];
}

fn combine_bytes(bytes: [u8;2]) -> u16 {
    let short_u16 = ((bytes[0] as u16) << 8) | bytes[1] as u16;
    short_u16
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub text: String,
    pub username: String, 
    pub ip: [u8;4],
    pub port: u16,
}

impl Message {
    pub fn new(text: String, username: String, ip: &[u8;4], port: &u16,) -> Self {
        Self {
            text,
            username,
            ip: *ip,
            port: *port,
        }
    }
    
    pub fn new_empty() -> Self {
        Self {
            text: "".to_string(),
            username: "".to_string(),
            ip: [0,0,0,0],
            port: 0,
        }
    }
}

/// # Generates the `Vec<u8>` buffer to send through the websocket from a message struct instance
/// first 4 bytes are ip, next two are port next byte is the length of the username in bytes and
/// then the the username and then length of the message text in bytes is two bytes
/// - bytes 0-3: ip
/// - bytes 4-5: port
/// - byte 6: username length
/// - username, max 255 bytes
/// - two bytes, text length
/// - rest: text theoretical max 65535
pub fn generate_message(message: Message) -> Vec<u8> {
    let mut message_buffer: Vec<u8> = Vec::new();
    message.ip.iter().for_each(|part| {
        message_buffer.push(*part);
    });
    let port_arr: [u8;2] = split_u16(message.port);
    message_buffer.append(&mut port_arr.to_vec());
    message_buffer.push(message.username.len() as u8);
    message_buffer.append(&mut message.username.as_bytes().to_vec());
    let text_len_arr: [u8;2] = split_u16(message.text.len() as u16);
    message_buffer.append(&mut text_len_arr.to_vec());
    message_buffer.append(&mut message.text.as_bytes().to_vec());
    
    message_buffer
}

/// # Turns a `Vec<u8>` buffer into a message struct instance
pub fn decypher_message(message: &Vec<u8>) -> Message {
    let mut message_out: Message = Message::new_empty();
    let mut temp_ip: [u8;4] = [0;4];
    for (i,msg_byte) in message[0..=3].iter().enumerate() {
        temp_ip[i] = *msg_byte;
    }

    message_out.ip = temp_ip;
    message_out.port = combine_bytes([message[4], message[5]]);

    let username_len = message[6];
    let mut temp_username: String = String::new();
    for username_byte in message[7..(7+username_len) as usize].iter() {
        temp_username.push(*username_byte as char);
    }

    // let text_len: usize = combine_bytes([message[(7+username_len) as usize], message[(7+username_len+1) as usize]]) as usize;
    let mut temp_text: String = String::new();
    for text_byte in message[(9+username_len as usize)..].iter() {
        temp_text.push(*text_byte as char);
    }

    message_out.username = temp_username;
    message_out.text = temp_text;

    message_out
}

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn start(address: Ipv4Addr, port: u16, _buffer_capacity: usize) -> Result<Self, Error> {
        Ok(Self {
            // message_buffer: { Arc::new(Mutex::new(VecDeque::with_capacity(buffer_capacity))) },
            stream: { TcpStream::connect(SocketAddrV4::new(address, port))? },
        })
    }

    pub fn send_message(&mut self, message: &Vec<u8>) -> Result<(), Error> {
        self.stream.write_all(&message.len().to_be_bytes())?;
        self.stream.write_all(message)?;
        self.stream.flush()?;
        
        Ok(())
    }

    pub fn receive(&mut self) -> Result<(), Error> {
        let mut stream_clone: TcpStream = self.stream.try_clone()?;
        thread::spawn(move|| -> Result<(), Error> {
            loop {
                let mut len_bytes: [u8;8] = [0_u8;8];
                stream_clone.read_exact(&mut len_bytes)?;
                let msg_len: usize = u64::from_be_bytes(len_bytes) as usize;
                let mut msg_buffer: Vec<u8> = vec![0_u8;msg_len];
                stream_clone.read_exact(&mut msg_buffer)?;
                let message: Message = decypher_message(&msg_buffer);
                println!("{:?}", message);
            }
        });

        Ok(())
    }
}

fn main() {
    let mut client = Client::start(Ipv4Addr::new(192,168,1,1), 3333, 80).expect("failed to create client");
    client.receive().unwrap();
    print!("> ");
    io::stdout().flush().unwrap();
    for line in io::stdin().lines() {
        print!("> ");
        io::stdout().flush().unwrap();
        let new_msg = Message::new(line.expect("failed to get input"), "deweyhinni".to_string(), &[192,168,1,2], &3333);
        client.send_message(&generate_message(new_msg)).unwrap();
    }
}

#[test]
fn test_buffer_stuff() {
    let message = Message::new("lmao idk what im doing".to_string(), "deweyhinni".to_string(), &[192_u8, 168_u8, 1_u8, 1_u8], &3333);
    let message_buff = generate_message(message.clone());
    let new_message = decypher_message(&message_buff);
    assert_eq!(message, new_message);
}
