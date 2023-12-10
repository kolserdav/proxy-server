use crate::http::CRLF;
use regex::Regex;
use std::{fmt, str};

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Debug)]
pub struct Headers {
    pub content_length: usize,
    pub raw: String,
    pub parsed: Vec<Header>,
}

impl Headers {
    pub fn new(buffer: Vec<u8>) -> Self {
        let raw = stringify(&buffer);
        let mut content_length: usize = 0;
        let content_length_op = get_content_length(&raw);
        if let Some(val) = content_length_op {
            content_length = val;
        }
        Headers {
            content_length,
            raw: raw.clone(),
            parsed: parse(raw),
        }
    }

    pub fn from_string(raw: String) -> Self {
        let mut content_length: usize = 0;
        let content_length_op = get_content_length(&raw);
        if let Some(val) = content_length_op {
            content_length = val;
        }
        Headers {
            content_length,
            raw: raw.clone(),
            parsed: parse(raw),
        }
    }

    pub fn change_host(self, target: &str) -> Self {
        let raw = change_host(self.raw, target);
        Headers {
            raw: raw.clone(),
            parsed: parse(raw),
            content_length: self.content_length,
        }
    }
}

/// Parse headers
fn parse(heads: String) -> Vec<Header> {
    let mut res: Vec<Header> = vec![];
    let heads = heads.split(CRLF);
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

/// Parse content length from request headers
pub fn get_content_length(src: &String) -> Option<usize> {
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
    let num = num_str.parse::<usize>();
    if let Err(e) = num {
        println!("Failed parse content lenght from str: {}: {}", num_str, e);
        return None;
    }
    Some(num.unwrap())
}