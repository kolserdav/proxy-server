use proxy_server::Builder;

fn main() {
    Builder::new().bind().expect("Error in proxy");
}
