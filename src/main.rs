use proxy_server::{log::LogLevel, Builder, ChangeTarget};

fn get_actual_target(old: &str) -> &'static str {
    let target1 = "127.0.0.1:3002";
    let target2 = "127.0.0.1:3003";

    match old {
        target1 => target2,
        target2 => target1,
    }
}

fn main() {
    let cb: ChangeTarget = (|old| get_actual_target(old));

    Builder::new()
        .with_address("127.0.0.1:3000")
        .with_target("127.0.0.1:3001")
        .with_log_level(LogLevel::Info)
        .with_threads(4)
        .with_change_target(cb)
        .bind()
        .expect("Error in proxy");
}
