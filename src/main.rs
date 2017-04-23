mod tiny_web_server;
fn main() {
    tiny_web_server::listener::listen("0.0.0.0:8423");
}
