use super::log::{Log, LogLevel};
use super::prelude::constants::*;
///! Module [`Http`].
///! The minimum set of methods to work through [`TcpStream`].
use super::prelude::*;
use std::{
    fmt,
    fmt::{Display, Formatter},
    io::{ErrorKind, Read, Result, Write},
    net::TcpStream,
    str,
};

/// End of line constant ([`\r\n`])
pub const CRLF: &str = "\r\n";
/// Version of HTTP protocol ([`HTTP/1.1`])
pub const VERSION: &str = "HTTP/1.1";

/// HTTP statuses enum
#[derive(Debug)]
pub enum Status {
    #[allow(dead_code)]
    OK,
    BadRequest,
    BadGateway,
}

impl Status {
    /// Get code value of HTTP status
    fn code(&self) -> u16 {
        use Status::*;
        match self {
            OK => 200,
            BadRequest => 400,
            BadGateway => 502,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let self_str = format!("{:?}", self);
        let res = space_bef_cap(self_str);
        write!(f, "{}", res)
    }
}

#[derive(Debug)]
pub struct Http {
    socket: TcpStream,
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

    /// Write HTTP status to response
    pub fn set_status(&mut self, status: Status) -> Result<usize> {
        self.write(format!("{} {}{}{}", VERSION, status.code(), status, CRLF).as_bytes())
    }

    /// Write content length header
    pub fn set_content_length<T>(&mut self, len: T) -> Result<usize>
    where
        T: Sized + std::fmt::Debug,
    {
        self.write(format!("Content-Length: {:?}{CRLF}", len).as_bytes())
    }

    /// Write end line to socket
    pub fn set_end_line(&mut self) -> Result<usize> {
        self.write(format!("{CRLF}").as_bytes())
    }

    /// Read request body
    pub fn read_body(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
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
        Ok(buf.len())
    }

    /// Client - Target tunnel core
    pub fn tunnel(&mut self, http: &mut Self, _log: &Log) -> Result<usize> {
        let mut size: usize = 0;
        loop {
            let mut b = [0; CHUNK_SIZE];
            let r_res = http.read(&mut b);
            if let Err(e) = r_res {
                let log_l = match e.kind() {
                    ErrorKind::ConnectionReset => LogLevel::Info,
                    _ => LogLevel::Error,
                };
                _log.println(log_l, "Failed read chunk", e);
            }
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
    pub fn read_headers(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        let mut size = 0;
        loop {
            let mut b = [0; 1];
            let len = self.read(&mut b)?;
            if len == 0 {
                break;
            }
            let b = b[0];
            let len = buf.len();
            size += 1;
            if len > 2
                && b == 10
                && (buf[len - 1] == 10 || (buf[len - 1] == 13 && buf[len - 2] == 10))
            {
                buf.push(b);
                break;
            }
            buf.push(b);
        }
        Ok(size)
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
