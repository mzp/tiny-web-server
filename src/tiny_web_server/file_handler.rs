use std::fs::File;
use std::io::{Read, Result};
use tiny_web_server::{request, response};
use std::path::PathBuf;

fn read_all(path : PathBuf) -> Result<String> {
    let mut buffer = String::new();
    return File::open(path)
        .and_then(|mut f| f.read_to_string(&mut buffer))
        .map(|_| buffer);
}

pub fn handle(request : &request::Request) -> Option<response::Response> {
    read_all(request::path(request))
        .map(response::ok)
        .ok()
}
