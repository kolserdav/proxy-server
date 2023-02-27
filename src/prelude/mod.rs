use regex::Regex;
use std::str;
pub mod constants;
use constants::*;

pub fn change_header_host(heads: &[u8]) -> Option<String> {
    let str_h = str::from_utf8(&heads).expect(
        "Fai
led parse incoming headers",
    );
    let reg = Regex::new(r"Host: *.*\r\n").unwrap();
    let capts = reg.captures(str_h);
    if let None = capts {
        return None;
    }
    let capts = capts.unwrap();
    let old_host = capts.get(0).unwrap().as_str();
    let heads_str = str::from_utf8(heads).expect(
        "
Failed stringify heads",
    );
    Some(heads_str.replace(old_host, format!("Host: {}\r\n", TARGET_ADDRESS).as_str()))
}

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
            }
        }
        res.push(v);
    }
    res
}
