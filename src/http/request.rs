///! Module [`Request`]
use crate::http::headers::Headers;
#[cfg(feature = "napi")]
use napi_derive::napi;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{io::Result, str};

/// HTTP request
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub url: String,
    pub host: String,
    pub peer_addr: String,
    pub protocol: String,
    pub method: String,
    pub content_length: u32,
    pub ttl: u32,
    pub headers: Headers,
    pub body: String,
    pub query: String,
    pub error: String,
    pub chunked: bool,
}

pub struct Socket {
    pub host: String,
    pub peer_addr: String,
    pub ttl: u32,
    pub error: String,
}

impl Request {
    pub fn new(socket: Socket, buffer: Vec<u8>) -> Result<Self> {
        let headers = Headers::from_bytes(&buffer)?;
        Ok(Request::create(socket, headers))
    }

    pub fn create(
        Socket {
            host,
            peer_addr,
            ttl,
            error,
        }: Socket,
        headers: Headers,
    ) -> Self {
        let mut content_length: u32 = 0;
        let content_length_op = Headers::get_content_length(&headers.raw);
        if let Some(val) = content_length_op {
            content_length = val;
        }
        Request {
            host,
            peer_addr,
            url: Headers::get_url(&headers.raw),
            protocol: Headers::get_protocol(&headers.raw),
            method: Headers::get_method(&headers.raw),
            content_length,
            ttl,
            body: "".to_string(),
            query: Headers::get_query(&headers.raw),
            error,
            chunked: Headers::get_chunked(&headers.raw),
            headers,
        }
    }

    pub fn change_host(&mut self, target: &str) -> Result<()> {
        let heads = self.headers.set_header("host", target)?;

        self.headers = heads;
        self.host = target.to_string();

        Ok(())
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
}

#[allow(dead_code)]
fn get_status(raw: &String) -> u16 {
    let reg = Regex::new(r"\d{3}").unwrap();
    let capts = reg.captures(raw.as_str());
    let mut status: u16 = 500;
    if let None = capts {
        return status;
    }
    let capts = capts.unwrap();
    let status_r = capts.get(0).unwrap().as_str().parse::<u16>();
    if let Ok(val) = status_r {
        status = val;
    }
    status
}

#[allow(dead_code)]
fn get_status_text(raw: &String) -> String {
    let reg = Regex::new(r"\d{3}[ \w\-]+").unwrap();
    let capts = reg.captures(raw.as_str());
    let mut status_text: String = "Internal Server Error".to_string();
    if let None = capts {
        return status_text;
    }
    let capts = capts.unwrap();
    status_text = capts.get(0).unwrap().as_str().to_string();
    status_text = Regex::new(r"^\d{3}\s+")
        .unwrap()
        .replace_all(&status_text, "")
        .to_string();
    status_text = Regex::new(r"^\s+")
        .unwrap()
        .replace_all(&status_text, "")
        .to_string();
    Regex::new(r"\s+$")
        .unwrap()
        .replace_all(&status_text, "")
        .to_string()
}
