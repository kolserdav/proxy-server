use std::{
    io::{ErrorKind, Read, Result, Write},
    net::{TcpListener, TcpStream},
    str,
    thread::sleep,
    time::Duration,
};
mod thread_pool;
use thread_pool::ThreadPool;
mod http;
use http::{Http, Status};
mod log;
use log::{Log, LogLevel};
mod prelude;
use prelude::*;

#[cfg(test)]
mod tests;

const CHUNK_SIZE: usize = 1024;

pub static TARGET_ADDRESS: &str = "127.0.0.1:3001";
pub static THREADS: usize = 4;
pub static LOG_LEVEL: LogLevel = LogLevel::Info;
pub static PROXY_ADDRESS: &str = "127.0.0.1:3000";

fn handle_proxy(client: TcpStream) -> Result<()> {
    let _log = Log::new(&LOG_LEVEL);
    _log.println(LogLevel::Info, "handle proxy", &client);

    let mut client = Http::from(client);
    let mut heads = vec![];
    client.read_to_end(&mut heads)?;
    let heads_n = change_header_host(&heads);
    if let None = heads_n {
        _log.println(LogLevel::Warn, "Header host is missing", &heads);
        client.set_status(Status::BadRequest);
        client.write("Content-Length: 0\r\n\r\n".as_bytes())?;
        return Ok(());
    }
    let heads_n = heads_n.unwrap();

    let http = Http::connect(TARGET_ADDRESS);
    if let Err(e) = &http {
        _log.println(LogLevel::Warn, "Failed proxy", e);
        client.set_status(Status::BadGateway)?;
        client.write("Content-Length: 0\r\n\r\n".as_bytes())?;
        client.flush()?;
        sleep(Duration::from_millis(100));
        return Ok(());
    }
    let mut http = http?;

    _log.println(LogLevel::Info, "write headers to target", &heads_n);
    http.write(heads_n.as_bytes())?;
    let mut h = vec![];
    http.read_to_end(&mut h)?;

    _log.println(
        LogLevel::Info,
        "send headers to client",
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
            _log.println(log_l, "Failed read chunk", e);
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
            handle_proxy(stream.expect("Error in incoming stream")).expect("Failed handle proxy");
        });
    }
    Ok(())
}

fn main() {
    proxy(PROXY_ADDRESS).expect("Error in proxy");
}
