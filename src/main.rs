mod tiny_web_server;
fn main() {
    println!("start tiny web server: 0.0.0.0:8423");
    tiny_web_server::listener::listen("0.0.0.0:8423");
}
