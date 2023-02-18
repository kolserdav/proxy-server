use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    str,
    sync::mpsc::{channel, Receiver, Sender},
    thread::spawn,
};
mod headers;
use headers::Headers;

fn handle_proxy(client: &mut TcpStream) -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3001")?;
    stream.write("GET / HTTP.1.1\r\nHost: 127.0.0.1:3001\r\n\r\n".as_bytes())?;
    let mut heads = Headers::new(&mut stream);
    let mut h = vec![];
    heads.read_to_end(&mut h)?;
    client.write(&h)?;
    let (tx, rx) = channel();
    spawn(move || loop {
        let mut b = [0; 1];
        let len = stream.read(&mut b).unwrap();
        if len == 0 {
            break;
        }
        tx.send(b).unwrap();
    });
    for r in rx {
        println!("{:?}:{:?}", &str::from_utf8(&r), &r);
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
        handle_target(&mut stream?)?;
    }
    Ok(())
}

fn handle_target(client: &mut TcpStream) -> Result<()> {
    const ECHO: [char; 5] = ['e', 'c', 'h', 'o', '\n'];
    println!("{:?}", client);
    let (tx, rx) = channel();
    tx.send("HTTP/1.1 200 OK\r\nContent-Type: plain/text\r\nTransfer-Encoding: chunked\r\nServer: echo-rs\r\n\r\n".as_bytes()).unwrap();
    spawn(move || {
        for i in 0..4 {
            tx.send("e".as_bytes()).unwrap();
        }
        tx.send("\r\n\r\n".as_bytes()).unwrap();
    });
    for r in rx {
        client.write(r)?;
    }
    Ok(())
}
