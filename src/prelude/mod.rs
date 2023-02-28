use regex::Regex;
use std::str;
pub mod constants;
use constants::*;

/// For change request headers host to host of target
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
    let low = Regex::new(r"content-length:\s*\d+").unwrap().captures(&src);

    let mut check: Option<&str> = None;
    if let Some(v) = low {
        let low = v.get(0).unwrap();
        check = Some(low.as_str());
    } else {
        let up = Regex::new(r"Content-Length:\s*\d+").unwrap().captures(&src);
        if let Some(_v) = up {
            let up = _v.get(0).unwrap();
            check = Some(up.as_str());
        } else {
            return None;
        }
    };

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
