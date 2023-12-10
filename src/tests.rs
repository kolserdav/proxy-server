use crate::headers::Headers;

#[cfg(test)]
use super::{
    http::{Http, Status, CRLF},
    log::{Log, LogLevel},
    Builder,
};
use std::{
    io::{Result, Write},
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
    let _log = Log::new(&super::LOG_LEVEL);

    let server = Builder::new();
    spawn(move || {
        target(server.target).expect("Error in target");
    });
    spawn(move || {
        server.bind(None).expect("Error in proxy");
    });
    sleep(Duration::from_secs(1));

    let mut http = Http::connect(server.address)?;
    http.write(format!("POST / HTTP/1.1{CRLF}Host: {}{CRLF}", server.address).as_bytes())?;
    http.set_content_length(ECHO.len())?;
    http.set_end_line()?;

    let mut t_v = vec![];
    let mut body: String = "".to_string();
    for l in ECHO {
        let d_str = l.clone().to_string();
        t_v.push(d_str);
        body.push(l);
    }

    http.write(body.as_bytes())?;
    http.write(&[0u8])?;

    let buff = http.read_headers()?;
    let heads = Headers::new(buff);
    _log.println(LogLevel::Info, "target read headers", &heads.parsed);
    let sp = heads.raw.split(CRLF).filter(|d| !d.is_empty());
    let mut v = vec![];
    let mut i = 0;
    for s in sp {
        i += 1;
        if i % 2 == 0 {
            v.push(s.to_string());
        }
    }

    assert_eq!(t_v, v);
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
    _log.println(LogLevel::Info, "Handle target start client", &client);

    let req_heads = client.read_headers()?;
    let heads = Headers::new(req_heads);
    _log.println(LogLevel::Info, "Handle target headers", &heads.parsed);

    let res_heads = format!(
        "Content-Type: plain/text{CRLF}Transfer-Encoding: chunked{CRLF}Server: echo-rs{CRLF}"
    );

    client.set_status(Status::OK)?;
    client.write(res_heads.as_bytes())?;
    client.set_end_line()?;

    if heads.content_length != 0 {
        let body = client.read_body()?;
        _log.println(
            LogLevel::Info,
            "Handle target body: ",
            str::from_utf8(&body).unwrap(),
        );
        for i in body {
            let chunk = format!("1{CRLF}{}{CRLF}", str::from_utf8(&[i]).unwrap());
            client.write(chunk.as_bytes())?;
        }
    }

    client.set_zero_byte()?;
    client.flush()?;
    _log.println(LogLevel::Info, "Handler target end client", client);
    Ok(())
}

#[test]
fn test_change_header_host() {
    let heads = Headers::from_string(format!("Host: {}{CRLF}", super::PROXY_ADDRESS));
    let heads = heads.change_host(super::TARGET_ADDRESS);
    assert_eq!(heads.raw, format!("Host: {}{CRLF}", super::TARGET_ADDRESS));
}
