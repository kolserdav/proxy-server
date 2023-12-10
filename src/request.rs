use crate::{headers::Headers, http::Http};
use regex::Regex;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Request {
    pub method: String,
    pub content_length: usize,
    pub headers: Headers,
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: "OPTIONS".to_string(),
            content_length: 0,
            headers: Headers {
                buffer: vec![],
                raw: "".to_string(),
                parsed: vec![],
            },
        }
    }

    pub fn get(self, client: &mut Http) -> Self {
        let mut content_length: usize = 0;
        let h_r = client.read_headers();
        if let Err(err) = h_r {
            println!("Failed set request {:?}", err);
            return self;
        }

        let h = h_r.unwrap();
        let heads = Headers::new(h);

        let content_length_op = get_content_length(&heads.raw);
        if let Some(val) = content_length_op {
            content_length = val;
        }

        Request {
            method: "OPTIONS".to_string(),
            content_length,
            headers: Headers {
                raw: heads.raw,
                parsed: heads.parsed,
                buffer: heads.buffer,
            },
        }
    }
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
