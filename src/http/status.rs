///! Module [`StatusDefault`]
use std::str;

use regex::Regex;

use crate::{http::CRLF, prelude::constants::HTTP_VERSION_DEFAULT};

/// HTTP status
#[derive(Debug)]
pub struct Status {
    pub text: String,
    pub code: u16,
}

/// HTTP defaul status
#[derive(Debug)]
pub struct StatusDefault {
    pub text: &'static str,
    pub code: u16,
}

/// HTTP statuses
/// Reference https://developer.mozilla.org/en-US/docs/Web/HTTP/StatusDefault
pub const STATUSES: [StatusDefault; 61] = [
    StatusDefault {
        text: "Continue",
        code: 100,
    },
    StatusDefault {
        text: "Switching Protocols",
        code: 101,
    },
    StatusDefault {
        text: "Processing",
        code: 102,
    },
    StatusDefault {
        text: "Early Hints",
        code: 103,
    },
    StatusDefault {
        text: "OK",
        code: 200,
    },
    StatusDefault {
        text: "Created",
        code: 201,
    },
    StatusDefault {
        text: "Accepted",
        code: 202,
    },
    StatusDefault {
        text: "Non-Authoritative Information",
        code: 203,
    },
    StatusDefault {
        text: "No Content",
        code: 204,
    },
    StatusDefault {
        text: "Reset Content",
        code: 205,
    },
    StatusDefault {
        text: "Partial Content",
        code: 206,
    },
    StatusDefault {
        text: "Multi-StatusDefault",
        code: 207,
    },
    StatusDefault {
        text: "Already Reported",
        code: 208,
    },
    StatusDefault {
        text: "IM Used",
        code: 226,
    },
    StatusDefault {
        text: "Multiple Choices",
        code: 300,
    },
    StatusDefault {
        text: "Moved Permanently",
        code: 301,
    },
    StatusDefault {
        text: "Found",
        code: 302,
    },
    StatusDefault {
        text: "See Other",
        code: 303,
    },
    StatusDefault {
        text: "Not Modified",
        code: 304,
    },
    StatusDefault {
        text: "Temporary Redirect",
        code: 307,
    },
    StatusDefault {
        text: "Permanent Redirect",
        code: 308,
    },
    StatusDefault {
        text: "Bad Request",
        code: 400,
    },
    StatusDefault {
        text: "Unauthorized",
        code: 401,
    },
    StatusDefault {
        text: "Payment Required",
        code: 402,
    },
    StatusDefault {
        text: "Forbidden",
        code: 403,
    },
    StatusDefault {
        text: "Not Found",
        code: 404,
    },
    StatusDefault {
        text: "Method Not Allowed",
        code: 405,
    },
    StatusDefault {
        text: "Not Acceptable",
        code: 406,
    },
    StatusDefault {
        text: "Proxy Authentication Required",
        code: 407,
    },
    StatusDefault {
        text: "Request Timeout",
        code: 408,
    },
    StatusDefault {
        text: "Conflict",
        code: 409,
    },
    StatusDefault {
        text: "Gone",
        code: 410,
    },
    StatusDefault {
        text: "Length Required",
        code: 411,
    },
    StatusDefault {
        text: "Precondition Failed",
        code: 412,
    },
    StatusDefault {
        text: "Payload Too Large",
        code: 413,
    },
    StatusDefault {
        text: "URI Too Long",
        code: 414,
    },
    StatusDefault {
        text: "Unsupported Media Type",
        code: 415,
    },
    StatusDefault {
        text: "Range Not Satisfiable",
        code: 416,
    },
    StatusDefault {
        text: "Expectation Failed",
        code: 417,
    },
    StatusDefault {
        text: "I'm a teapot",
        code: 418,
    },
    StatusDefault {
        text: "Misdirected Request",
        code: 421,
    },
    StatusDefault {
        text: "Unprocessable Content",
        code: 422,
    },
    StatusDefault {
        text: "Locked",
        code: 423,
    },
    StatusDefault {
        text: "Failed Dependency",
        code: 424,
    },
    StatusDefault {
        text: "Upgrade Required",
        code: 426,
    },
    StatusDefault {
        text: "Precondition Required",
        code: 428,
    },
    StatusDefault {
        text: "Too Many Requests",
        code: 429,
    },
    StatusDefault {
        text: "Request Header Fields Too Large",
        code: 431,
    },
    StatusDefault {
        text: "Request Header Fields Too Large",
        code: 431,
    },
    StatusDefault {
        text: "Unavailable For Legal Reasons",
        code: 451,
    },
    StatusDefault {
        text: "Internal Server Error",
        code: 500,
    },
    StatusDefault {
        text: "Not Implemented",
        code: 501,
    },
    StatusDefault {
        text: "Bad Gateway",
        code: 502,
    },
    StatusDefault {
        text: "Service Unavailable",
        code: 503,
    },
    StatusDefault {
        text: "Gateway Timeout",
        code: 504,
    },
    StatusDefault {
        text: "HTTP Version Not Supported",
        code: 505,
    },
    StatusDefault {
        text: "Variant Also Negotiates",
        code: 506,
    },
    StatusDefault {
        text: "Insufficient Storage",
        code: 507,
    },
    StatusDefault {
        text: "Loop Detected",
        code: 508,
    },
    StatusDefault {
        text: "Not Extended",
        code: 510,
    },
    StatusDefault {
        text: "Network Authentication Required",
        code: 511,
    },
];

impl Status {
    /// Create HTTP status from code
    pub fn new(code: u16) -> Status {
        let mut status = Status {
            text: "Not Implemented".to_string(),
            code: 501,
        };
        for f in STATUSES {
            if code == f.code {
                status = Status {
                    code: f.code,
                    text: f.text.to_string(),
                };
            }
        }
        status
    }

    /// Get HTTP protocol prefix like `HTTP/1.1 200 OK`
    pub fn to_full_string(&self) -> String {
        let d = self.to_string();
        let res = Regex::new(format!(r"{CRLF}$").as_str())
            .unwrap()
            .replace_all(d.as_str(), "");
        format!("{HTTP_VERSION_DEFAULT} {}", res)
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.code, self.text)
    }
}
