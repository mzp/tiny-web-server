use std::net::TcpStream;
use std::io::{Read, Error};
use std::result::{Result};

fn pred(ref c : &Result<u8, Error>) -> bool {
    c.map(|ch| ch != ('\n' as u8))
     .unwrap_or(false)
}

fn read_header(stream : TcpStream) -> String {
    let mut buffer = String::new();
    for s in stream.bytes().take_while(|ch| pred(ch)) {
        buffer.push(char::from(s.unwrap()));
    }
    return buffer;
}

pub fn handle_client(stream : TcpStream) {
    read_header(stream);
}
