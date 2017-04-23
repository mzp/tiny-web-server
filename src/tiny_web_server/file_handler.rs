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

pub fn handle<'a>(request : request::Request) -> Option<response::Response<'a>> {
    let path = current_dir().unwrap().join(request.path);
    //read_all(path).map(|s : String| response::ok(s)).ok()
    None
}
