use regex::Regex;
use std::{
    io::{Result, Write},
    net::{TcpListener, TcpStream},
    str,
};

use crate::{
    http::{Http, Status, CRLF},
    log::{Log, LogLevel},
    request::Request,
};
pub mod constants;

/// Set spaces before capitalize letters. For change [`Http::Status`] enum items.
pub fn space_bef_cap(src: String) -> String {
    let chars = src.chars().into_iter();
    let mut res = "".to_string();
    for v in chars {
        let mut buf = [0; 2];
        v.encode_utf8(&mut buf);
        let ch = Regex::new(r"[A-Z]{1}")
            .unwrap()
            .captures(&str::from_utf8(&buf).unwrap());
        if let Some(_) = ch {
            if src != "OK" {
                res.push(' ');
            } else if v == 'O' {
                res.push(' ')
            }
        }
        res.push(v);
    }
    res
}

pub fn target(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening target on {}", addr);
    for stream in listener.incoming() {
        handle_target(stream?)?;
    }
    Ok(())
}

pub fn handle_target(client: TcpStream) -> Result<()> {
    const TAG: &str = "Handle target";
    let _log = Log::new(&super::LOG_LEVEL);
    let mut client = Http::from(client);
    _log.println(LogLevel::Info, TAG, "start client", &client);

    let req_heads = client.read_headers()?;
    let req = Request::new(req_heads);
    _log.println(LogLevel::Info, TAG, "headers", &req.headers);

    let res_heads = format!(
        "Content-Type: plain/text{CRLF}Transfer-Encoding: chunked{CRLF}Server: echo-rs{CRLF}"
    );

    client.set_status(Status::OK)?;
    client.write(res_heads.as_bytes())?;
    client.set_end_line()?;

    if req.content_length != 0 {
        let body = client.read_body(&req)?;
        _log.println(LogLevel::Info, TAG, "body", str::from_utf8(&body).unwrap());
        for i in body {
            let chunk = format!("1{CRLF}{}{CRLF}", str::from_utf8(&[i]).unwrap());
            client.write(chunk.as_bytes())?;
        }
    }

    client.set_zero_byte()?;
    client.flush()?;
    _log.println(LogLevel::Info, TAG, "end client", client);
    Ok(())
}
