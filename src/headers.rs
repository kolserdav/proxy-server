use std::{
    io::{Read, Result},
    net::TcpStream,
};

pub struct Headers<'a> {
    socket: &'a mut TcpStream,
}

impl<'a> Headers<'a> {
    pub fn new(socket: &'a mut TcpStream) -> Headers<'a> {
        Headers { socket }
    }
}

impl<'a> Read for Headers<'a> {
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
