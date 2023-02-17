use std::io;
use std::io::{Result, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

fn echo_main(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    for mut stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    println!("listening on {}", addr);
    Ok(())
}

fn handle_client(client: &mut TcpStream) {
    println!("{:?}", client);
    client.write("HTTP/1.1 200 OK\r\nContent-Length: 6\r\nContent-Type: plain/text\r\nAccept-Ranges: bytes\r\nServer: tes\r\n\r\necho\r\n".as_bytes());
}

fn main() {
    echo_main("127.0.0.1:3000").expect("error: ");
}
