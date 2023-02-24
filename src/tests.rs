use super::http::Http;
use super::log::{Log, LogLevel};
use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    str,
};

#[cfg(test)]
use std::{
    thread::{sleep, spawn},
    time::Duration,
};

#[cfg(test)]
const ECHO: [char; 4] = ['e', 'c', 'h', 'o'];

#[test]
fn test() -> Result<()> {
    spawn(move || {
        target(super::TARGET_ADDRESS).expect("Error in target");
    });
    spawn(move || {
        super::proxy(super::PROXY_ADDRESS).expect("Error in proxy");
    });
    sleep(Duration::from_secs(1));
    let mut http = Http::connect(super::PROXY_ADDRESS)?;
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
    let _log = Log::new(&super::LOG_LEVEL);
    _log.println(LogLevel::Info, "handle target", &client);

    client.write("HTTP/1.1 200 OK\r\nContent-Type: plain/text\r\nTransfer-Encoding: chunked\r\nServer: echo-rs\r\n\r\n".as_bytes()).unwrap();
    for i in ECHO {
        let chunk = format!("1\r\n{}\r\n", i);
        client.write(chunk.as_bytes())?;
    }

    client.write("0\r\n\r\n".as_bytes())?;
    client.flush()?;
    sleep(Duration::from_millis(100));

    Ok(())
}
