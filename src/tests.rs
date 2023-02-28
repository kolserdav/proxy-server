#[cfg(test)]
use super::{
    http::{Http, Status, CRLF},
    log::{Log, LogLevel},
    prelude::*,
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
        server.bind().expect("Error in proxy");
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

    let mut buff: Vec<u8> = vec![];
    http.read_headers(&mut buff)?;
    _log.println(
        LogLevel::Info,
        "target read headers",
        str::from_utf8(&buff).unwrap(),
    );
    buff = vec![];
    http.read_headers(&mut buff)?;

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
    _log.println(LogLevel::Info, "handle target", &client);

    let mut req_heads = vec![];
    client.read_headers(&mut req_heads)?;
    let req_heads = str::from_utf8(&req_heads).unwrap();
    _log.println(LogLevel::Info, "read headers on target", &req_heads);

    let res_heads = format!(
        "Content-Type: plain/text{CRLF}Transfer-Encoding: chunked{CRLF}Server: echo-rs{CRLF}"
    );

    client.set_status(Status::OK)?;
    client.write(res_heads.as_bytes())?;
    client.set_end_line()?;

    let cont_len = get_content_length(&format!("{:?}", req_heads));
    if let Some(v) = cont_len {
        if v != 0 {
            let mut body = vec![];
            client.read_body(&mut body)?;
            _log.println(
                LogLevel::Info,
                "request body on target: ",
                str::from_utf8(&body).unwrap(),
            );
            for i in body {
                let chunk = format!("1{CRLF}{}{CRLF}", str::from_utf8(&[i]).unwrap());
                client.write(chunk.as_bytes())?;
            }
        }
    } else {
        _log.println(LogLevel::Warn, "get_content_length return", cont_len);
    }

    client.write("0{CRLF}{CRLF}".as_bytes())?;
    sleep(Duration::from_millis(200));
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
