use std::{
    io::{ErrorKind, Read, Result, Write},
    net::{TcpListener, TcpStream},
    str,
    thread::{sleep, spawn},
    time::Duration,
};
mod thread_pool;
use thread_pool::ThreadPool;
mod headers;
use headers::Headers;
mod http;
use http::Http;
mod log;
use log::{Log, LogLevel};

const CHUNK_SIZE: usize = 1024;
static LOG_LEVEL: LogLevel = LogLevel::Info;

fn handle_proxy(client: &mut TcpStream) -> Result<()> {
    let _log = Log::new(&LOG_LEVEL);

    _log.println(LogLevel::Info, "handle proxy", &client);

    let mut stream = TcpStream::connect("127.0.0.1:3001")?;
    stream.write("GET / HTTP.1.1\r\nHost: 127.0.0.1:3001\r\n\r\n".as_bytes())?;
    let mut heads = Headers::new(&mut stream);
    let mut h = vec![];
    heads.read_to_end(&mut h)?;

    _log.println(
        LogLevel::Info,
        "send headers to clint",
        str::from_utf8(&h).expect("failed decode bytes"),
    );
    client.write(&h).expect("failed send headers");

    loop {
        let mut b = [0; CHUNK_SIZE];
        let r_res = stream.read(&mut b);
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
        client.write(&buf).expect("failed send chunk from thread");
    }
    Ok(())
}

fn proxy(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    let _log = Log::new(&LOG_LEVEL);
    println!("listening proxy on {}", addr);

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        pool.execute(|| {
            handle_proxy(&mut stream.expect("error in stream of tcp listener"))
                .expect("failed handle proxy");
        });
    }
    Ok(())
}

fn main() {
    spawn(move || {
        target("127.0.0.1:3001").expect("error target");
    });
    proxy("127.0.0.1:3000").expect("error proxy");
}

fn target(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening target on {}", addr);
    for stream in listener.incoming() {
        handle_target(&mut stream?)?;
    }
    Ok(())
}

fn handle_target(client: &mut TcpStream) -> Result<()> {
    let _log = Log::new(&LOG_LEVEL);
    const ECHO: [char; 5] = ['e', 'c', 'h', 'o', '\n'];
    _log.println(LogLevel::Info, "handle target", &client);

    let http = Http::new();
    client.write("HTTP/1.1 200 OK\r\nContent-Type: plain/text\r\nTransfer-Encoding: chunked\r\nServer: echo-rs\r\n\r\n".as_bytes()).unwrap();
    for i in 0..8 {
        let chunk = http.chunk("e");
        client.write(chunk.as_bytes()).unwrap();
    }
    client.write("0\r\n\r\n".as_bytes()).unwrap();
    sleep(Duration::from_millis(100));
    Ok(())
}
