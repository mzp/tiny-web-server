use std::net::{TcpListener, ToSocketAddrs};

pub fn listen<A: ToSocketAddrs>(addr: A) {
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("new client!");
            }
            Err(_) => { /* connection failed */ }
        }
    }
}
