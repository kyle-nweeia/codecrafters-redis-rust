use std::{
    io::{Result, Write},
    net::TcpListener,
};

fn main() -> Result<()> {
    for stream in TcpListener::bind("127.0.0.1:6379").unwrap().incoming() {
        match stream {
            Ok(mut stream) => {
                stream.write_all("+PONG\r\n".as_bytes())?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
