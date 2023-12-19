pub mod headers;
pub mod request;
pub mod status;
use self::request::Request;

use super::log::Log;
use super::prelude::constants::*;
///! Module [`Http`].
///! The minimum set of methods to work through [`TcpStream`].
use regex::Regex;
use std::{
    io::{Error, ErrorKind, Read, Result, Write},
    net::TcpStream,
    str,
};

/// End of line constant ([`\r\n`])
pub const CRLF: &str = "\r\n";

#[derive(Debug)]
pub struct Http {
    pub socket: TcpStream,
}

impl Http {
    /// Create [`Http`] with new TCP connection
    pub fn connect(address: &str) -> Result<Http> {
        let socket = TcpStream::connect(address)?;
        Ok(Http::from(socket))
    }

    /// Create [`Http`] from exists socket
    pub fn from(socket: TcpStream) -> Http {
        Http { socket }
    }

    /// Write end of request
    pub fn set_zero_byte(&mut self) -> Result<usize> {
        self.write(format!("0{CRLF}{CRLF}").as_bytes())
    }

    /// Read request body
    pub fn read_body(&mut self, req: &Request) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];
        if req.content_length == 0 && !req.chunked {
            return Ok(vec![]);
        }
        loop {
            let mut chunk = [0; super::CHUNK_SIZE];
            self.read(&mut chunk)?;
            let mut exit = false;
            'b: for ch in chunk {
                if ch == 0 {
                    exit = true;
                    break 'b;
                }
                buf.push(ch);
            }
            if exit == true {
                break;
            }
        }
        Ok(buf)
    }

    /// Body to string
    pub fn body_to_string(&mut self, body: Vec<u8>) -> Result<String> {
        let res = str::from_utf8(&body);
        if let Err(err) = res {
            println!("{}", err);
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to parse body to string",
            ));
        }
        let result = res.unwrap();
        let rec = Regex::new(r"\d*\r\n(0$)?").unwrap().replace_all(result, "");
        Ok(rec.to_string())
    }

    /// Client - Target tunnel core
    pub fn tunnel(&mut self, http: &mut Self, _log: &Log) -> Result<usize> {
        let mut size: usize = 0;
        loop {
            let mut b = [0; CHUNK_SIZE];
            http.read(&mut b)?;
            let mut buf = vec![];
            b.map(|_b| {
                if _b != 0 {
                    buf.push(_b);
                    return true;
                }
                false
            });

            size += buf.len();

            if buf.len() == 0 {
                break;
            }

            self.write(&buf)?;
        }

        Ok(size)
    }

    /// Read request headers by one byte for fist empty line
    pub fn read_headers(&mut self) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];
        loop {
            let mut b = [0; 1];
            let len = self.read(&mut b)?;
            if len == 0 {
                break;
            }
            let b = b[0];
            let len = buf.len();
            if len > 2
                && b == 10
                && (buf[len - 1] == 10 || (buf[len - 1] == 13 && buf[len - 2] == 10))
            {
                buf.push(b);
                break;
            }
            buf.push(b);
        }
        Ok(buf)
    }
}

impl Read for Http {
    /// Read chunk bytes from request
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.socket.read(buf)
    }
}

impl Write for Http {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.socket.write(data)
    }

    fn flush(&mut self) -> Result<()> {
        self.socket.flush()
    }
}
