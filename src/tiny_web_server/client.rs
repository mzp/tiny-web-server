use std::net::TcpStream;
use  std::io::Read;

pub fn handle_client(mut stream : TcpStream) {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer);
    println!("{}", buffer);
}
