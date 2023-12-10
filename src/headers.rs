use crate::{http::CRLF, request::get_content_length};
use regex::Regex;
use serde::Serialize;
use std::{fmt, str};

#[derive(Debug, Serialize, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Headers {
    pub buffer: Vec<u8>,
    pub raw: String,
    pub parsed: Vec<Header>,
}

impl Headers {
    pub fn new(buffer: Vec<u8>) -> Self {
        let raw = stringify(&buffer);

        Headers {
            buffer: buffer.clone(),
            raw: raw.clone(),
            parsed: parse(&raw),
        }
    }

    pub fn from_string(raw: String) -> Self {
        Headers {
            buffer: vec![],
            raw: raw.clone(),
            parsed: parse(&raw),
        }
    }

    pub fn change_host(&mut self, target: &str) {
        let raw = change_host(self.raw.clone(), target);
        let parsed = parse(&raw);
        self.raw = raw;
        self.parsed = parsed;
    }
}

/// Parse headers
fn parse(heads: &String) -> Vec<Header> {
    let mut res: Vec<Header> = vec![];
    let heads = heads.split(CRLF);
    println!("{:?}", &heads);
    for h in heads {
        // TODO check it reg
        let reg_name = Regex::new(r"^.+: ").unwrap();
        let capts_name = reg_name.captures(h);
        if let None = capts_name {
            continue;
        }
        let capts_name = capts_name.unwrap();
        let name = capts_name
            .get(0)
            .unwrap()
            .as_str()
            .to_string()
            // FIXME
            .replace(": ", "");

        let reg_value = Regex::new(r": *.*$").unwrap();
        let capts_value = reg_value.captures(h);
        if let None = capts_value {
            res.push(Header {
                name,
                value: "".to_string(),
            });
            continue;
        }
        let capts_value = capts_value.unwrap();
        let value = capts_value
            .get(0)
            .unwrap()
            .as_str()
            .to_string()
            .replace(": ", "");
        res.push(Header { name, value });
    }
    res
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
