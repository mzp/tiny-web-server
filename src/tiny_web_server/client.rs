use std::net::TcpStream;
use std::io::{Read, Error};
use std::result::{Result};

fn pred(c : &Result<u8, Error>) -> bool {
    match c {
        &Ok(ch) => { ch != ('\n' as u8) }
        &Err(_) => { false }
    }
}

fn read_header(stream : TcpStream) -> String {
    let mut buffer = String::new();
    for s in stream.bytes().take_while(|ch| pred(ch)) {
        buffer.push(char::from(s.unwrap()));
    }
    return buffer;
}

pub fn handle_client(stream : TcpStream) {
    let header = read_header(stream);
    println!("{}", header);
}
