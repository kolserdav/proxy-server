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
use std::{
    thread::{sleep, spawn},
    time::Duration,
};

const CHUNK_SIZE: usize = 1024;

#[cfg(test)]
const ECHO: [char; 4] = ['e', 'c', 'h', 'o'];

static TARGET_ADDRESS: &str = "127.0.0.1:3001";
static THREADS: usize = 4;
static LOG_LEVEL: LogLevel = LogLevel::Info;
static PROXY_ADDRESS: &str = "127.0.0.1:3000";

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
    // test();
}

#[test]
fn test() -> Result<()> {
    spawn(move || {
        target(TARGET_ADDRESS).expect("Error in target");
    });
    spawn(move || {
        proxy(PROXY_ADDRESS).expect("Error in proxy");
    });
    sleep(Duration::from_secs(1));
    let mut http = Http::connect(PROXY_ADDRESS)?;
    let mut buff: Vec<u8> = vec![];
    http.read_to_end(&mut buff)?;
    buff = vec![];
    http.read_to_end(&mut buff)?;
    let res = str::from_utf8(&buff).unwrap();
    let sp = res.split("\r\n").filter(|d| !d.is_empty());
    let mut v = vec![];
    let mut i = 0;
    for s in sp {
        i += 1;
        if i % 2 == 0 {
            v.push(s.to_string());
        }
    }
    let mut t_v = vec![];
    for l in ECHO {
        t_v.push(l.clone().to_string());
    }
    assert_eq!(t_v, v);
    Ok(())
}

#[cfg(test)]
fn target(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening target on {}", addr);
    for stream in listener.incoming() {
        handle_target(&mut stream?)?;
    }
    Ok(())
}

#[cfg(test)]
fn handle_target(client: &mut TcpStream) -> Result<()> {
    let _log = Log::new(&LOG_LEVEL);
    _log.println(LogLevel::Info, "handle target", &client);

    client.write("HTTP/1.1 200 OK\r\nContent-Type: plain/text\r\nTransfer-Encoding: chunked\r\nServer: echo-rs\r\n\r\n".as_bytes()).unwrap();
    for i in ECHO {
        let chunk = format!("1\r\n{}\r\n", i);
        client.write(chunk.as_bytes()).unwrap();
    }

    client.write("0\r\n\r\n".as_bytes()).unwrap();
    sleep(Duration::from_millis(100));

    Ok(())
}
