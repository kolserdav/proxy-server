use crate::http::CRLF;
#[allow(unused_imports)]
use napi_derive::napi;
use regex::Regex;
use serde::Serialize;
use std::fmt;

#[cfg_attr(feature = "napi", napi(object))]
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

impl Header {
    /// Parse headers
    pub fn parse(heads: String) -> Vec<Header> {
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
            res.push(Header {
                name,
                value: value.to_lowercase(),
            });
        }
        res
    }
}
