use tiny_web_server::{request, response};
use std::path::PathBuf;

fn read_dir(path : PathBuf) -> String {
    let mut buffer = String::new();

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            println!("{:?}", entry.path());
        }
    }

    return buffer;
}

pub fn handle(request : &request::Request) -> Option<response::Response> {
    read_dir(request::path(request));
/*    read_all(
        .map(response::ok)
        .ok()*/
    None
}
