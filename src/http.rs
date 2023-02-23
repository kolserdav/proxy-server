use std::{
    io::{Read, Result, Write},
    net::TcpStream,
};

pub struct Http {
    socket: TcpStream,
}

impl Http {
    pub fn connect(address: &str) -> Result<Http> {
        let mut socket = TcpStream::connect(address)?;
        Ok(Http { socket })
    }
}

impl Read for Http {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.socket.read(buf)
    }

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
