use std::env::current_dir;
use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use tiny_web_server::{request, response};

fn read_all(path : PathBuf) -> Result<String> {
    let mut buffer = String::new();
    return File::open(path)
        .and_then(|mut f| f.read_to_string(&mut buffer))
        .map(|_| buffer);
}

fn resolve(path : String) -> PathBuf {
    current_dir().unwrap().join(".".to_string() + &path)
}

pub fn handle<'a>(request : request::Request) -> Option<response::Response> {
    read_all(resolve(request.path))
        .map(response::ok)
        .ok()
}
