use proxy_server::Builder;

fn main() {
    Builder::new()
        .with_address("192.168.0.3:3000")
        .bind()
        .expect("Error in proxy");
}
