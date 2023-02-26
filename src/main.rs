use proxy_server::{log::LogLevel, Builder};

fn main() {
    Builder::new()
        .with_address("127.0.0.2:3000")
        .with_target("127.0.0.1:3001")
        .with_log_level(LogLevel::Warn)
        .with_threads(4)
        .bind()
        .expect("Error in proxy");
}
