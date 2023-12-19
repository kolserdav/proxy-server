///! Module [`Header`]
use crate::http::CRLF;
#[cfg(feature = "napi")]
use napi_derive::napi;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    io::{Error, ErrorKind, Result},
    str,
};

use super::status::Status;

/// HTTP header
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
}

/// HTTP headers
#[cfg_attr(feature = "napi", napi(object))]
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Headers {
    pub raw: String,
    pub list: Vec<Header>,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

impl Headers {
    pub fn new() -> Self {
        Headers::from_string("".to_string())
    }

    // Create request headers
    pub fn new_request(prefix: &str, list: Vec<Header>) -> Self {
        let postfix = Headers::to_string(list);
        let raw = format!(
            "{}{CRLF}{postfix}",
            Regex::new(r"\s*$")
                .unwrap()
                .replace_all(prefix, "")
                .to_string()
        );
        Headers::from_string(raw)
    }

    /// Create response headers
    pub fn new_response(status: &Status, list: Vec<Header>) -> Self {
        let postfix = Headers::to_string(list);
        let prefix = status.to_full_string();
        let raw = format!("{prefix}{CRLF}{postfix}");
        Headers::from_string(raw)
    }

    /// Parse headers
    pub fn from_string(raw: String) -> Self {
        let mut res: Headers = Headers {
            raw: raw.clone(),
            list: vec![],
        };
        let heads = raw.split(CRLF);
        for h in heads {
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
                .replace(": ", "");

            let reg_value = Regex::new(r": *.*$").unwrap();
            let capts_value = reg_value.captures(h);
            if let None = capts_value {
                res.list.push(Header {
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
            res.list.push(Header {
                name,
                value: value.to_lowercase(),
            });
        }
        res
    }

    /// Create string of headers from list
    pub fn to_string(list: Vec<Header>) -> String {
        let mut result = "".to_string();
        for h in list {
            result = format!(
                "{result}{}: {}{CRLF}",
                h.name.to_lowercase(),
                h.value.to_lowercase()
            );
        }
        result = format!("{result}{CRLF}");
        result
    }

    /// Create headers from bytes
    pub fn from_bytes(heads: &Vec<u8>) -> Result<Self> {
        let res = str::from_utf8(heads);
        if let Err(err) = res {
            return Err(Error::new(ErrorKind::InvalidInput, err));
        }
        let res = res.unwrap().to_string();
        Ok(Headers::from_string(res))
    }

    fn change_header(&self, name: &str, value: &str) -> (Vec<Header>, bool) {
        let mut check = false;
        let mut new_list: Vec<Header> = vec![];
        for h in self.list.clone() {
            if name.to_lowercase() == h.name.as_str().to_lowercase() {
                new_list.push(Header {
                    name: name.to_string(),
                    value: value.to_string(),
                });
                check = true;
            } else {
                new_list.push(Header {
                    name: h.name.to_string(),
                    value: h.value.to_string(),
                });
            }
        }
        (new_list, check)
    }

    /// Set new header or change old one
    pub fn set_header(&self, name: &str, value: &str) -> Result<Self> {
        let (mut new_list, check) = self.change_header(name, value);
        if !check {
            new_list.push(Header {
                name: name.to_string(),
                value: value.to_string(),
            });
        }

        let new_h = match self.is_response() {
            true => {
                let status = Headers::get_status(&self.raw)?;
                Headers::new_response(&status, new_list)
            }
            false => {
                let prefix = Headers::get_headers_prefix(&self.raw)?;
                Headers::new_request(prefix.as_str(), new_list)
            }
        };

        Ok(Headers {
            list: new_h.list,
            raw: new_h.raw,
        })
    }

    /// Parse content length from request headers
    pub fn get_content_length(raw: &String) -> Option<u32> {
        let low = Regex::new(r"(c|C)ontent-(l|L)ength:\s*\d+")
            .unwrap()
            .captures(&raw);

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

    /// Get url from raw headers
    pub fn get_url(raw: &String) -> String {
        let reg = Regex::new(r"\/[a-zA-Z0-9_\-\/]*").unwrap();
        let capts = reg.captures(raw.as_str());
        if let None = capts {
            return "/".to_string();
        }
        let capts = capts.unwrap();
        let res = capts.get(0).unwrap().as_str();
        res.to_string()
    }

    /// Get query string from raw headers
    pub fn get_query(raw: &String) -> String {
        let reg = Regex::new(r"\?[a-zA-Z0-9_\-&=\.]*").unwrap();
        let capts = reg.captures(raw.as_str());
        if let None = capts {
            return "".to_string();
        }
        let capts = capts.unwrap();
        let res = capts.get(0).unwrap().as_str();
        res.to_string()
    }

    // Get protocol from raw headers
    pub fn get_protocol(raw: &String) -> String {
        let reg = Regex::new(r"HTTPS?\/\d+\.\d+").unwrap();
        let capts = reg.captures(raw.as_str());
        if let None = capts {
            return "OPTIONS".to_string();
        }
        let capts = capts.unwrap();
        let protocol = capts.get(0).unwrap().as_str();
        protocol.to_string()
    }

    // Get request prefix
    fn get_status(raw: &String) -> Result<Status> {
        let reg = Regex::new(format!(r".+{CRLF}").as_str()).unwrap();
        let capts = reg.captures(raw.as_str());
        if let None = capts {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Headers prefix didn't find",
            ));
        }
        let capts = capts.unwrap();
        let result = capts.get(0).unwrap().as_str();

        let result = Regex::new(format!(r"^HTTPS?\/\d+\.\d+ ").as_str())
            .unwrap()
            .replace_all(result, "")
            .to_string();

        let reg = Regex::new(format!(r"^\d+").as_str()).unwrap();
        let capts = reg.captures(result.as_str());
        if let None = capts {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Headers prefix didn't find",
            ));
        }
        let capts = capts.unwrap();
        let code = capts.get(0).unwrap().as_str().parse::<u16>();
        if let Err(err) = code {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Failed parse status code {:?}", err),
            ));
        }
        let code = code.unwrap();

        let text = Regex::new(format!(r"^\d+ ").as_str())
            .unwrap()
            .replace_all(result.as_str(), "")
            .to_string();

        Ok(Status { code, text })
    }

    /// Get method from raw headers
    pub fn get_method(raw: &String) -> String {
        let reg = Regex::new(r"\w+").unwrap();
        let capts = reg.captures(raw.as_str());
        if let None = capts {
            return "OPTIONS".to_string();
        }
        let capts = capts.unwrap();
        let method = capts.get(0).unwrap().as_str();
        method.to_string()
    }

    // Get request prefix
    fn get_headers_prefix(raw: &String) -> Result<String> {
        let reg = Regex::new(format!(r".+{CRLF}").as_str()).unwrap();
        let capts = reg.captures(raw.as_str());
        if let None = capts {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Headers prefix didn't find",
            ));
        }
        let capts = capts.unwrap();
        let result = capts.get(0).unwrap().as_str();
        let result = Regex::new(format!(r"{CRLF}+$").as_str())
            .unwrap()
            .replace_all(result, "");
        Ok(result.to_string())
    }

    fn is_response(&self) -> bool {
        let reg = Regex::new(r"^HTTPS?/\d+\.\d+").unwrap();
        let capts = reg.captures(self.raw.as_str());
        if let None = capts {
            return false;
        }
        true
    }
}
