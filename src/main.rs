use std::{
    io::{ErrorKind, Read, Result, Write},
    net::{TcpListener, TcpStream},
    str,
};
mod thread_pool;
use thread_pool::ThreadPool;
mod http;
use http::Http;
mod log;
use log::{Log, LogLevel};

#[cfg(test)]
mod tests;

const CHUNK_SIZE: usize = 1024;

pub static TARGET_ADDRESS: &str = "127.0.0.1:3001";
pub static THREADS: usize = 4;
pub static LOG_LEVEL: LogLevel = LogLevel::Info;
pub static PROXY_ADDRESS: &str = "127.0.0.1:3000";

fn handle_proxy(client: &mut TcpStream) -> Result<()> {
    let _log = Log::new(&LOG_LEVEL);

    _log.println(LogLevel::Info, "handle proxy", &client);
    let mut http = Http::connect(TARGET_ADDRESS)?;
    http.write("GET / HTTP.1.1\r\nHost: 127.0.0.1:3001\r\n\r\n".as_bytes())?;
    let mut h = vec![];
    http.read_to_end(&mut h)?;

    _log.println(
        LogLevel::Info,
        "send headers to clint",
        str::from_utf8(&h).expect("failed decode bytes"),
    );
    client.write(&h).expect("failed send headers");

    loop {
        let mut b = [0; CHUNK_SIZE];
        let r_res = http.read(&mut b);
        if let Err(e) = r_res {
            let log_l = match e.kind() {
                ErrorKind::ConnectionReset => LogLevel::Info,
                _ => LogLevel::Error,
            };
            _log.println(log_l, "failed read chunk", e);
        }
        let mut buf = vec![];
        b.map(|_b| {
            if _b != 0 {
                buf.push(_b);
                return true;
            }
            false
        });
        if buf.len() == 0 {
            break;
        }
        client.write(&buf).expect("Failed write chunk");
    }
    Ok(())
}

fn proxy(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;

    let _log = Log::new(&LOG_LEVEL);
    println!("Listening proxy on {}", addr);

    let pool = ThreadPool::new(THREADS);

    for stream in listener.incoming() {
        pool.execute(|| {
            handle_proxy(&mut stream.expect("Error in incoming stream"))
                .expect("Failed handle proxy");
        });
    }
    Ok(())
}

fn main() {
    proxy(PROXY_ADDRESS).expect("Error in proxy");
}
