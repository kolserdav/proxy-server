use crate::http::headers::Header;
use crate::http::request::Socket;
use crate::http::{headers::Headers, request::Request};
use crate::prelude::constants::{HTTP_VERSION_DEFAULT, TTL_DEFAULT};
use crate::prelude::target;

#[cfg(test)]
use super::{
    http::{Http, CRLF},
    log::{Log, LogLevel},
    Builder,
};
use std::io::{Result, Write};
use std::{
    thread::{sleep, spawn},
    time::Duration,
};

const ECHO: [char; 4] = ['e', 'c', 'h', 'o'];
const TAG: &str = "Test proxy server";

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
    let send_body = t_v.join("");

    http.write(body.as_bytes())?;
    http.write(&[0u8])?;

    let buff = http.read_headers()?;
    let req = Request::new(
        Socket {
            host: server.address.to_string(),
            peer_addr: server.address.to_string(),
            ttl: TTL_DEFAULT,
            error: "".to_string(),
        },
        buff,
    )?;
    _log.println(LogLevel::Info, TAG, "request", &req);

    let b = http.read_body(&req)?;
    let rec_body = http.body_to_string(b)?;
    _log.println(LogLevel::Info, TAG, "body: ", &rec_body);

    assert_eq!(send_body, rec_body);
    Ok(())
}

#[test]
fn test_change_header_host() -> Result<()> {
    let headers = Headers::new(
        HTTP_VERSION_DEFAULT,
        vec![Header {
            name: "Host".to_string(),
            value: super::PROXY_ADDRESS.to_string(),
        }],
    );
    let mut req = Request::create(
        Socket {
            host: super::PROXY_ADDRESS.to_string(),
            peer_addr: super::PROXY_ADDRESS.to_string(),
            ttl: TTL_DEFAULT,
            error: "".to_string()
        },
        headers,
    );
    req.change_host(super::TARGET_ADDRESS)?;

    assert_eq!(
        req.headers.raw,
        format!(
            "{HTTP_VERSION_DEFAULT}{CRLF}host: {}{CRLF}{CRLF}",
            super::TARGET_ADDRESS
        )
    );
    Ok(())
}
