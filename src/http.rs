///! Module [`Http`].
///! The minimum set of methods to work through [`TcpStream`].
use super::prelude::*;
use std::{
    fmt,
    fmt::{Display, Formatter},
    io::{Read, Result, Write},
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

    /// Read request body
    pub fn read_body(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        loop {
            let mut chunk = [0; super::CHUNK_SIZE];
            self.read(&mut chunk)?;
            let mut exit = false;
            'b: for ch in chunk {
                if ch == 0 {
                    buf.push(ch);
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
}

impl Read for Http {
    /// Read chunk bytes from request
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.socket.read(buf)
    }

    /// Read request headers by one byte for fist empty line
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
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

impl Write for Http {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.socket.write(data)
    }

    fn flush(&mut self) -> Result<()> {
        self.socket.flush()
    }
}
