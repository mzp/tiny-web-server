use std::net::TcpStream;
use std::io::{Read, Result};
use tiny_web_server::{request, response, file_handler};

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

pub fn handle_client(mut stream : TcpStream) {
    let request = read_request(&stream);
    println!("{:?}", request);

    let response =
        file_handler::handle(request)
        .unwrap_or_else(|| response::not_found());

    response::write(&mut stream, response);
}
