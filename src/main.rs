use std::io::{Read, Result, Write};

fn main() -> Result<()> {
    let mut buf = [0; 512];

    for result in std::net::TcpListener::bind("127.0.0.1:6379")?.incoming() {
        match result {
            Ok(mut stream) => {
                while stream.read(&mut buf)? != 0 {
                    stream.write_all(b"+PONG\r\n")?;
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
