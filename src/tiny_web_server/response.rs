use std::net::TcpStream;
use std::io::{Write};

#[derive(Debug)]
pub enum Status {
    OK,
    NotFound
}

#[derive(Debug)]
pub struct Response {
    pub status : Status,
    pub body : String
}

pub fn ok(body : String) -> Response {
    Response{
        status: Status::OK,
        body: body
    }
}

pub fn not_found() -> Response {
    Response{
        status: Status::NotFound,
        body: "not found".to_string()
    }
}

fn write_line(stream : &mut TcpStream, line : &[u8]) {
    let _ = stream.write(line);
    let _ = stream.write(b"\r\n");
}

pub fn write(stream : &mut TcpStream, response : Response) {
    match response.status {
        Status::OK => { write_line(stream, b"HTTP/1.1 200 OK") }
        Status::NotFound => { write_line(stream, b"HTTP/1.1 404 Not found") }
    }
    write_line(stream, b"");
    write_line(stream, response.body.as_bytes());
}
