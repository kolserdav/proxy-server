use std::io;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening on {}", addr);
    loop {
        let (mut stream, peer) = listener.accept()?;
        println!("{:?}", stream);
        let mut write_stream = stream.try_clone()?;
        /*
                let buf = format!(
                    "200 OK\r\n
                    GET {0} HTTP/1.1\r\n\
                 Host: {0}\r\n\
                 \r\n",
                    &addr
                )
                .into_bytes();

                write_stream.write(&buf);
        */
        let mut f = std::fs::File::open("./src/main.rs").expect("err 78");
        spawn(move || {
            io::copy(&mut f, &mut write_stream).expect("err: 88 ");
        });
    }
}
fn main() {
    echo_main("127.0.0.1:3000").expect("error: ");
}
