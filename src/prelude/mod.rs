use crate::http::CRLF;
use regex::Regex;
use std::{fmt, str};
pub mod constants;

/// For change request headers host to host of target
pub fn change_header_host(heads: &str, target: &str) -> Option<String> {
    let reg = Regex::new(r"Host: *.*\r\n").unwrap();
    let capts = reg.captures(heads);
    if let None = capts {
        return None;
    }
    let capts = capts.unwrap();
    let old_host = capts.get(0).unwrap().as_str();
    Some(heads.replace(old_host, format!("Host: {}\r\n", target).as_str()))
}

#[derive(Debug)]
pub struct Header {
    name: String,
    value: String,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

/// Parse headers
pub fn parse_headers(heads: String) -> Vec<Header> {
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
pub fn stringify_headers(heads: &Vec<u8>) -> String {
    let s = str::from_utf8(heads);
    match s {
        Ok(val) => val.to_string(),
        Err(err) => {
            println!("Failed to stringify headers: {:?}", err);
            "".to_string()
        }
    }
}

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
