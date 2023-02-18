use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    str,
    sync::mpsc::channel,
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
    println!(
        "{:?} (read_timeout: {:?})",
        &str::from_utf8(&h),
        stream.read_timeout()
    );
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
    client.flush();
    Ok(())
}

fn proxy(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        handle_proxy(&mut stream?)?;
    }
    println!("listening proxy on {}", addr);
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
    for stream in listener.incoming() {
        handle_target(&mut stream?)?;
    }
    println!("listening target on {}", addr);
    Ok(())
}

fn handle_target(client: &mut TcpStream) -> Result<()> {
    const ECHO: [char; 5] = ['e', 'c', 'h', 'o', '\n'];
    println!("{:?}", client);
    client.write("HTTP/1.1 200 OK\r\nContent-Length: 5\r\nContent-Type: plain/text\r\nAccept-Ranges: bytes\r\nServer: echo-rs\r\n\r\n".as_bytes())?;
    let (tx, rx) = channel();
    spawn(move || {
        for i in 0..ECHO.len() {
            tx.send(ECHO[i]).unwrap();
        }
    });
    for r in rx {
        client.write(r.to_string().as_bytes())?;
    }
    client.flush();
    Ok(())
}
