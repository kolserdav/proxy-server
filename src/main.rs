use std::{
    io::{Result, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::channel,
    thread::spawn,
};

fn echo_main(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    for mut stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    println!("listening on {}", addr);
    Ok(())
}

const ECHO: [char; 5] = ['e', 'c', 'h', 'o', '\n'];

fn handle_client(client: &mut TcpStream) -> Result<()> {
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

fn main() {
    echo_main("127.0.0.1:3000").expect("error: ");
}
