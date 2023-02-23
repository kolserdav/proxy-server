use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    str,
    sync::mpsc::channel,
    thread::{sleep, spawn},
    time::Duration,
};
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
    let (tx, rx) = channel();
    tx.send(h).expect("failed send headers");
    let c_log = _log.clone();
    spawn(move || loop {
        let mut b = [0; CHUNK_SIZE];
        let r_res = stream.read(&mut b);
        if let Err(e) = r_res {
            c_log.println(LogLevel::Error, "failed read chunk", e);
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
        tx.send(buf).expect("failed send chunk from thread");
    });
    for r in rx {
        _log.println(
            LogLevel::Info,
            "answer from target",
            str::from_utf8(&r).expect("failed decode bytes"),
        );
        client.write(&r)?;
    }
    Ok(())
}

fn proxy(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    let _log = Log::new(&LOG_LEVEL);
    println!("listening proxy on {}", addr);
    for stream in listener.incoming() {
        handle_proxy(&mut stream?)?;
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
    let (tx, rx) = channel();
    spawn(move || {
        let http = Http::new();
        tx.send("HTTP/1.1 200 OK\r\nContent-Type: plain/text\r\nTransfer-Encoding: chunked\r\nServer: echo-rs\r\n\r\n".as_bytes()).unwrap();
        let mut s = "".to_string();
        for i in 0..8 {
            tx.send(&http.chunk("e")).unwrap();
        }
        tx.send("0\r\n\r\n".as_bytes()).unwrap();
    });
    for r in rx {
        _log.println(
            LogLevel::Info,
            "send from target",
            str::from_utf8(&r).unwrap(),
        );
        client.write(r)?;
    }
    sleep(Duration::from_millis(100));
    Ok(())
}
