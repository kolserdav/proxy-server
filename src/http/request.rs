///! Module [`Request`]
use crate::http::headers::Headers;
#[allow(unused_imports)]
use napi_derive::napi;
use regex::Regex;
use serde::Serialize;
use std::{
    io::{Error, ErrorKind, Result},
    str,
};

/// HTTP request
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Debug, Serialize, Clone)]
pub struct Request {
    pub url: String,
    pub protocol: String,
    pub method: String,
    pub content_length: u32,
    pub headers: Headers,
    pub body: String,
}

impl Request {
    pub fn new(buffer: Vec<u8>) -> Result<Self> {
        let headers = Headers::from_bytes(&buffer)?;
        Ok(Request::create(headers))
    }

    pub fn create(headers: Headers) -> Self {
        let mut content_length: u32 = 0;
        let content_length_op = Headers::get_content_length(&headers.raw);
        if let Some(val) = content_length_op {
            content_length = val;
        }
        Request {
            url: Headers::get_url(&headers.raw),
            protocol: Headers::get_protocol(&headers.raw),
            method: Headers::get_method(&headers.raw),
            content_length,
            body: "".to_string(),
            headers,
        }
    }

    pub fn change_host(&mut self, target: &str) -> Result<()> {
        let heads = self.headers.change_header("host", target);
        if let None = heads {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Header host is missing",
            ));
        }
        let heads = heads.unwrap();
        self.headers = heads;
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
