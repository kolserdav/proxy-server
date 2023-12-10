use proxy_server::{log::LogLevel, prelude::target, Builder, ChangeTarget};
use std::thread::spawn;

fn get_dynamic_target(old: &'static str) -> &'static str {
    let target1 = "127.0.0.1:3001";
    let target2 = "127.0.0.1:3003";
    let res = match old {
        "127.0.0.1:3001" => target2,
        "127.0.0.1:3003" => target1,
        _ => target1,
    };
    res
}

fn get_static_target(old: &'static str) -> &'static str {
    old
}

fn main() {
    let ra = "127.0.0.1:3003";
    let cb: ChangeTarget = |old| get_static_target(old);

    spawn(move || {
        target(ra).expect("Error in target");
    });
    Builder::new()
        .with_address("127.0.0.1:3000")
        .with_target(ra)
        .with_log_level(LogLevel::Info)
        .with_threads(4)
        .bind(Some(cb))
        .expect("Error in proxy");
}
