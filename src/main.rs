#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{self, Write, Read};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted new connection");

                if let Err(e) = handle_connection(stream){
                    println!("error handling connection: {e}")
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_connection(mut stream: TcpStream) -> io::Result<()>
{
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf)?;
    println!("{}", String::from_utf8_lossy(&buf[..n]));

    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;
    Ok(())
}