use std::net::TcpStream;
use std::io::{Read, Write, Result};
use tiny_web_server::request;

fn read_line(stream : &TcpStream) -> String {
    let not_new_line = |c : &Result<u8>|
        match c {
            &Ok(ch) => { ch != ('\n' as u8) }
            &Err(_) => { false }
        };

    let mut buffer = String::new();
    for s in stream.bytes().take_while(not_new_line) {
        buffer.push(char::from(s.unwrap()));
    }
    return buffer;
}

fn read_request(stream : &TcpStream) -> request::Request {
    return request::parse(read_line(stream));
}

fn write_line(stream : &mut TcpStream, line : String) {
    let _ = stream.write(line.as_bytes());
    let _ = stream.write(b"\r\n");
}

pub fn handle_client(mut stream : TcpStream) {
    let header = read_request(&stream);
    println!("{:?}", header);

    write_line(&mut stream, "HTTP/1.1 200 OK".to_string());
    write_line(&mut stream, "".to_string());
    write_line(&mut stream, header.path);
}
