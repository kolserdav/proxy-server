#[cfg(test)]
use super::http::{Http, Status, CRLF};
use super::log::{Log, LogLevel};
use super::prelude::*;
use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    str,
};
use std::{
    thread::{sleep, spawn},
    time::Duration,
};

const ECHO: [char; 4] = ['e', 'c', 'h', 'o'];

#[test]
pub fn test_proxy_server() -> Result<()> {
    println!("Start test of proxy");
    spawn(move || {
        target(super::TARGET_ADDRESS).expect("Error in target");
    });
    spawn(move || {
        super::bind(super::PROXY_ADDRESS).expect("Error in proxy");
    });
    sleep(Duration::from_secs(1));
    let mut http = Http::connect(super::PROXY_ADDRESS)?;
    let mut buff: Vec<u8> = vec![];
    http.write(
        format!(
            "GET / HTTP/1.1{CRLF}Host: {}{CRLF}{CRLF}",
            super::TARGET_ADDRESS
        )
        .as_bytes(),
    )?;
    http.read_to_end(&mut buff)?;
    buff = vec![];
    http.read_to_end(&mut buff)?;
    let res = str::from_utf8(&buff).unwrap();
    let sp = res.split(CRLF).filter(|d| !d.is_empty());
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
    println!("Test of proxy is end: {:?}", t_v);
    Ok(())
}

pub fn target(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening target on {}", addr);
    for stream in listener.incoming() {
        handle_target(stream?)?;
    }
    Ok(())
}

fn handle_target(client: TcpStream) -> Result<()> {
    let _log = Log::new(&super::LOG_LEVEL);
    let mut client = Http::from(client);
    _log.println(LogLevel::Info, "handle target", &client);

    let heads = format!(
        "Content-Type: plain/text{CRLF}Transfer-Encoding: chunked{CRLF}Server: echo-rs{CRLF}{CRLF}"
    );
    _log.println(LogLevel::Info, "target write headers", &heads);
    client.set_status(Status::OK)?;
    client.write(heads.as_bytes())?;
    for i in ECHO {
        let chunk = format!("1{CRLF}{}{CRLF}", i);
        client.write(chunk.as_bytes())?;
    }

    client.write("0{CRLF}{CRLF}".as_bytes())?;
    sleep(Duration::from_millis(100));
    client.flush()?;
    _log.println(LogLevel::Info, "target return", client);
    Ok(())
}

#[test]
fn test_change_header_host() {
    let heads = format!("Host: {}{CRLF}", super::PROXY_ADDRESS);
    let head_n = change_header_host(heads.as_bytes());
    assert!(None != head_n);
    let head_n = head_n.unwrap();
    assert_eq!(head_n, format!("Host: {}{CRLF}", super::TARGET_ADDRESS));
}
