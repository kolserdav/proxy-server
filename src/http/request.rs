use crate::http::header::Header;
#[cfg(napi)]
use napi_derive::napi;
use regex::Regex;
use serde::Serialize;
use std::str;

cfg_if::cfg_if! {
    if #[cfg(napi)] {
        #[napi(object)]
    }
}
#[derive(Debug, Serialize, Clone)]
pub struct Request {
    pub url: String,
    pub protocol: String,
    pub method: String,
    pub content_length: u32,
    pub headers_raw: String,
    pub headers: Vec<Header>,
    pub body: String,
}

impl Request {
    pub fn new(buffer: Vec<u8>) -> Self {
        let raw = stringify(&buffer);
        Request::from_string(raw)
    }

    pub fn from_string(raw: String) -> Self {
        let mut content_length: u32 = 0;
        let content_length_op = get_content_length(&raw);
        if let Some(val) = content_length_op {
            content_length = val;
        }
        Request {
            url: get_url(&raw),
            protocol: get_protocol(&raw),
            method: get_method(&raw),
            content_length,
            body: "".to_string(),
            headers_raw: raw.clone(),
            headers: Header::parse(raw),
        }
    }

    pub fn change_host(&mut self, target: &str) {
        let raw = change_host(self.headers_raw.clone(), target);
        self.headers_raw = raw.clone();
        self.headers = Header::parse(raw);
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
}

/// Stringify headers
fn stringify(heads: &Vec<u8>) -> String {
    let s = str::from_utf8(heads);
    match s {
        Ok(val) => val.to_string(),
        Err(err) => {
            println!("Failed to stringify headers: {:?}", err);
            "".to_string()
        }
    }
}

/// For change request headers host to host of target
fn change_host(heads: String, target: &str) -> String {
    let reg = Regex::new(r"Host: *.*\r\n").unwrap();
    let capts = reg.captures(heads.as_str());
    if let None = capts {
        return heads;
    }
    let capts = capts.unwrap();
    let old_host = capts.get(0).unwrap().as_str();
    heads.replace(old_host, format!("Host: {}\r\n", target).as_str())
}

/// Parse content length from request headers
fn get_content_length(src: &String) -> Option<u32> {
    let low = Regex::new(r"(c|C)ontent-(l|L)ength:\s*\d+")
        .unwrap()
        .captures(&src);

    #[allow(unused_assignments)]
    let mut check: Option<&str> = None;
    if let Some(v) = low {
        let low = v.get(0).unwrap();
        check = Some(low.as_str());
    }

    if let None = check {
        return None;
    }

    let cont_len = check.unwrap();

    let num = Regex::new(r"\d+").unwrap().captures(cont_len);
    if let None = num {
        return None;
    }
    let capts = num.unwrap();
    let num = capts.get(0);
    let num_str = num.unwrap().as_str();
    let num = num_str.parse::<u32>();
    if let Err(e) = num {
        println!("Failed parse content lenght from str: {}: {}", num_str, e);
        return None;
    }
    Some(num.unwrap())
}

fn get_method(raw: &String) -> String {
    let reg = Regex::new(r"\w+").unwrap();
    let capts = reg.captures(raw.as_str());
    if let None = capts {
        return "OPTIONS".to_string();
    }
    let capts = capts.unwrap();
    let method = capts.get(0).unwrap().as_str();
    method.to_string()
}

fn get_url(raw: &String) -> String {
    let reg = Regex::new(r"\/[a-zA-Z0-9_\-\/]*").unwrap();
    let capts = reg.captures(raw.as_str());
    if let None = capts {
        return "/".to_string();
    }
    let capts = capts.unwrap();
    let url = capts.get(0).unwrap().as_str();
    url.to_string()
}

fn get_protocol(raw: &String) -> String {
    let reg = Regex::new(r"HTTPS?\/\d+\.\d+").unwrap();
    let capts = reg.captures(raw.as_str());
    if let None = capts {
        return "OPTIONS".to_string();
    }
    let capts = capts.unwrap();
    let protocol = capts.get(0).unwrap().as_str();
    protocol.to_string()
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
