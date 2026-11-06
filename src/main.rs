use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut buf = [0; 512];
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let mut stream = listener.accept().await?.0;

        tokio::spawn(async move {
            while stream.read(&mut buf).await? != 0 {
                stream.write_all(b"+PONG\r\n").await?;
            }

            Ok::<(), std::io::Error>(())
        });
    }
}
