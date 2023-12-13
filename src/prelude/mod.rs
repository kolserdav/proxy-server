use regex::Regex;
use std::{
    io::{Result, Write},
    net::{TcpListener, TcpStream},
    str,
};

use crate::{
    http::{
        headers::{Header, Headers},
        request::{Request, Socket},
        status::Status,
        Http, CRLF,
    },
    log::{Log, LogLevel},
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

    let error = client.socket.take_error().unwrap();
    let error = match error {
        None => "".to_string(),
        Some(val) => val.to_string(),
    };

    let req = Request::new(
        Socket {
            host: client.socket.local_addr().unwrap().to_string(),
            peer_addr: client.socket.peer_addr().unwrap().to_string(),
            ttl: client.socket.ttl().unwrap(),
            error,
        },
        req_heads,
    )?;
    _log.println(LogLevel::Info, TAG, "headers", &req.headers);

    let res_heads = Headers::new_response(
        &Status::new(200),
        vec![
            Header {
                name: "Content-Type".to_string(),
                value: "plain/text".to_string(),
            },
            Header {
                name: "Transfer-Encoding".to_string(),
                value: "chunked".to_string(),
            },
            Header {
                name: "Server".to_string(),
                value: "echo-rs".to_string(),
            },
        ],
    );

    client.write(res_heads.raw.as_bytes())?;

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
