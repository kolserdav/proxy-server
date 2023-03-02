use proxy_server::{log::LogLevel, Builder, ChangeTarget};

fn get_actual_target(old: &str) -> &'static str {
    let target1 = "127.0.0.1:3001";
    let target2 = "127.0.0.1:3003";
    let res = match old {
        "127.0.0.1:3001" => target2,
        "127.0.0.1:3003" => target1,
        _ => target1,
    };
    res
}

fn main() {
    let cb: ChangeTarget = |old| get_actual_target(old);

    Builder::new()
        .with_address("127.0.0.1:3000")
        .with_target("127.0.0.1:3003")
        .with_log_level(LogLevel::Info)
        .with_threads(4)
        .bind(Some(cb))
        .expect("Error in proxy");
}
