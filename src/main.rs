use std::{
    io::{Read, Result, Write},
    iter::FromIterator,
    net::{TcpListener, TcpStream},
    str,
    sync::mpsc::{channel, Receiver, Sender},
    thread::spawn,
};
mod headers;
use headers::Headers;

const CHUNK_SIZE: usize = 1024;

fn handle_proxy(client: &mut TcpStream) -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3001")?;
    stream.write("GET / HTTP.1.1\r\nHost: 127.0.0.1:3001\r\n\r\n".as_bytes())?;
    let mut heads = Headers::new(&mut stream);
    let mut h = vec![];
    heads.read_to_end(&mut h)?;
    println!("handle proxy {:?}", &client);
    let (tx, rx) = channel();
    tx.send(h).unwrap();
    spawn(move || loop {
        let mut b = [0; CHUNK_SIZE];
        let len = stream.read(&mut b).unwrap();
        if len == 0 {
            break;
        }
        let mut buf = vec![];
        #[allow(unused_must_use)]
        b.map(|_b| {
            if _b != 0 {
                buf.push(_b);
                return true;
            }
            false
        });
        if buf.len() == 0 {
            break;
        }
        tx.send(buf).unwrap();
    });
    for r in rx {
        println!("send to client: {:?}", str::from_utf8(&r).unwrap());
        client.write(&r)?;
    }
    Ok(())
}

fn proxy(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening proxy on {}", addr);
    for stream in listener.incoming() {
        handle_proxy(&mut stream?)?;
    }
    Ok(())
}

fn main() {
    spawn(move || {
        target("127.0.0.1:3001").expect("error target");
    });
    proxy("127.0.0.1:3000").expect("error proxy");
}

fn target(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening target on {}", addr);
    for stream in listener.incoming() {
        //       println!("start");
        handle_target(&mut stream?)?;
    }
    Ok(())
}

fn handle_target(client: &mut TcpStream) -> Result<()> {
    const ECHO: [char; 5] = ['e', 'c', 'h', 'o', '\n'];
    println!("handle target {:?}", client);
    let (tx, rx) = channel();
    spawn(move || {
        tx.send("HTTP/1.1 200 OK\r\nContent-Type: plain/text\r\nAccept-Ranges: bytes\r\nTransfer-Encoding: chunked\r\nServer: echo-rs\r\n\r\n".as_bytes()).unwrap();
        for i in 0..4 {
            tx.send("1\r\ne\r\n".as_bytes()).unwrap();
        }
        tx.send("0\r\n\r\n".as_bytes()).unwrap();
    });
    for r in rx {
        println!("send from target: {:?}", str::from_utf8(&r).unwrap());
        client.write(r)?;
    }
    // println!("end");
    Ok(())
}
