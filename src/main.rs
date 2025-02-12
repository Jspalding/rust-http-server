use std::{io::Write, net::TcpListener};

fn main() {
    println!("Logs");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                _stream.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("200 \n");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
