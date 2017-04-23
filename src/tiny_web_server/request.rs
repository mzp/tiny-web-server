#[derive(Debug)]
pub struct Request {
    method : String,
    path : String
}

pub fn parse(line : String) -> Request {
    let mut xs = line.split(' ');
    let method = xs.next().unwrap_or("").to_string();
    let path = xs.next().unwrap_or("").to_string();

    Request { method: method, path: path }
}
