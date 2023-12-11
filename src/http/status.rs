///! Module [`Status`]
use std::{
    fmt,
    fmt::{Display, Formatter},
    str,
};

/// HTTP status
#[derive(Debug)]
pub struct Status {
    pub name: &'static str,
    pub code: u16,
}

/// HTTP statuses
/// Reference https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
pub const STATUSES: [Status; 61] = [
    Status {
        name: "Continue",
        code: 100,
    },
    Status {
        name: "Switching Protocols",
        code: 101,
    },
    Status {
        name: "Processing",
        code: 102,
    },
    Status {
        name: "Early Hints",
        code: 103,
    },
    Status {
        name: "OK",
        code: 200,
    },
    Status {
        name: "Created",
        code: 201,
    },
    Status {
        name: "Accepted",
        code: 202,
    },
    Status {
        name: "Non-Authoritative Information",
        code: 203,
    },
    Status {
        name: "No Content",
        code: 204,
    },
    Status {
        name: "Reset Content",
        code: 205,
    },
    Status {
        name: "Partial Content",
        code: 206,
    },
    Status {
        name: "Multi-Status",
        code: 207,
    },
    Status {
        name: "Already Reported",
        code: 208,
    },
    Status {
        name: "IM Used",
        code: 226,
    },
    Status {
        name: "Multiple Choices",
        code: 300,
    },
    Status {
        name: "Moved Permanently",
        code: 301,
    },
    Status {
        name: "Found",
        code: 302,
    },
    Status {
        name: "See Other",
        code: 303,
    },
    Status {
        name: "Not Modified",
        code: 304,
    },
    Status {
        name: "Temporary Redirect",
        code: 307,
    },
    Status {
        name: "Permanent Redirect",
        code: 308,
    },
    Status {
        name: "Bad Request",
        code: 400,
    },
    Status {
        name: "Unauthorized",
        code: 401,
    },
    Status {
        name: "Payment Required",
        code: 402,
    },
    Status {
        name: "Forbidden",
        code: 403,
    },
    Status {
        name: "Not Found",
        code: 404,
    },
    Status {
        name: "Method Not Allowed",
        code: 405,
    },
    Status {
        name: "Not Acceptable",
        code: 406,
    },
    Status {
        name: "Proxy Authentication Required",
        code: 407,
    },
    Status {
        name: "Request Timeout",
        code: 408,
    },
    Status {
        name: "Conflict",
        code: 409,
    },
    Status {
        name: "Gone",
        code: 410,
    },
    Status {
        name: "Length Required",
        code: 411,
    },
    Status {
        name: "Precondition Failed",
        code: 412,
    },
    Status {
        name: "Payload Too Large",
        code: 413,
    },
    Status {
        name: "URI Too Long",
        code: 414,
    },
    Status {
        name: "Unsupported Media Type",
        code: 415,
    },
    Status {
        name: "Range Not Satisfiable",
        code: 416,
    },
    Status {
        name: "Expectation Failed",
        code: 417,
    },
    Status {
        name: "I'm a teapot",
        code: 418,
    },
    Status {
        name: "Misdirected Request",
        code: 421,
    },
    Status {
        name: "Unprocessable Content",
        code: 422,
    },
    Status {
        name: "Locked",
        code: 423,
    },
    Status {
        name: "Failed Dependency",
        code: 424,
    },
    Status {
        name: "Upgrade Required",
        code: 426,
    },
    Status {
        name: "Precondition Required",
        code: 428,
    },
    Status {
        name: "Too Many Requests",
        code: 429,
    },
    Status {
        name: "Request Header Fields Too Large",
        code: 431,
    },
    Status {
        name: "Request Header Fields Too Large",
        code: 431,
    },
    Status {
        name: "Unavailable For Legal Reasons",
        code: 451,
    },
    Status {
        name: "Internal Server Error",
        code: 500,
    },
    Status {
        name: "Not Implemented",
        code: 501,
    },
    Status {
        name: "Bad Gateway",
        code: 502,
    },
    Status {
        name: "Service Unavailable",
        code: 503,
    },
    Status {
        name: "Gateway Timeout",
        code: 504,
    },
    Status {
        name: "HTTP Version Not Supported",
        code: 505,
    },
    Status {
        name: "Variant Also Negotiates",
        code: 506,
    },
    Status {
        name: "Insufficient Storage",
        code: 507,
    },
    Status {
        name: "Loop Detected",
        code: 508,
    },
    Status {
        name: "Not Extended",
        code: 510,
    },
    Status {
        name: "Network Authentication Required",
        code: 511,
    },
];

impl Status {
    /// Get code value of HTTP status
    pub fn new(code: u16) -> Status {
        let mut status = Status {
            name: "Not Implemented",
            code: 501,
        };
        for f in STATUSES {
            if code == f.code {
                status = f;
            }
        }
        status
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", self.code, self.name)
    }
}
