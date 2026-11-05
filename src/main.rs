use std::net::TcpListener;

fn main() {
    for stream in TcpListener::bind("127.0.0.1:6379").unwrap().incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
