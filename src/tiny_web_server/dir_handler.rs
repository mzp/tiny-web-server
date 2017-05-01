use tiny_web_server::{request, response};
use std::path::PathBuf;
use std::path::Path;

fn read_dir(base : &String, path : PathBuf) -> Option<String> {
    path.read_dir()
        .map(|entries| entries
             .flat_map(|e| e.ok().and_then(|x| x.file_name().into_string().ok()))
             .map(|name|
                  format!("<li><a href='{}'>{}</a></li>", Path::new(base).join(&name).to_str().unwrap(), &name))
             .collect())
        .map(|content : String| format!(
                "<html>
                  <head><title>{}</title></head>
                  <body><ul>{}</ul></body>
                </html>", base, content))
        .ok()
}

pub fn handle(request : &request::Request) -> Option<response::Response> {
    read_dir(&request.path, request::path(request)).map(response::ok)
}
