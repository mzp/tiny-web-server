use std::net::{TcpListener, ToSocketAddrs};
use tiny_web_server::client;

pub fn listen<A: ToSocketAddrs>(addr: A) {
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream.map(|s| client::handle_client(s)) {
            Ok(_) => { /* successful */ }
            Err(why) => { println!("{}", why) }
        }
    }
}
