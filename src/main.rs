use std::{
    io::{Read, Result, Write},
    net::{Shutdown, TcpListener, TcpStream},
    str,
    sync::mpsc::channel,
    thread::spawn,
};

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
    Ok(())
}

fn handle_proxy(client: &mut TcpStream) -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3001")?;
    stream.write("GET / HTTP.1.1\r\nHost: 127.0.0.1:3001\r\n\r\n".as_bytes())?;
    let mut h = vec![];
    loop {
        let mut b = [0; 1];
        let len = stream.read(&mut b)?;
        if len == 0 {
            break;
        }
        let b = b[0];
        let len = h.len();
        if len > 2 && b == 10 && (h[len - 1] == 10 || (h[len - 1] == 13 && h[len - 2] == 10)) {
            h.push(b);
            break;
        }
        h.push(b);
    }
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
