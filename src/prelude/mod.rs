use regex::Regex;
use std::str;
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
