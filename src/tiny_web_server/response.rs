use std::net::TcpStream;
use std::io::{Write};

#[derive(Debug)]
pub enum Status {
    OK,
    NotFound
}

#[derive(Debug)]
pub struct Response<'a> {
    pub status : Status,
    pub body : &'a [u8]
}

pub fn ok<'a>(body : &'a String) -> Response<'a> {
    Response{
        status: Status::OK,
        body: body.as_bytes()
    }
}

pub fn not_found() -> Response<'static> {
    Response{
        status: Status::NotFound,
        body: b"not found"
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
    write_line(stream, response.body);
}
