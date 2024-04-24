use std::time::Duration;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener, time};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    while let Ok((mut socket, addr)) = listener.accept().await {
        tokio::spawn(async move {
            println!("Accepted connection from: {}", addr);
            let (mut rx, mut tx) = socket.split();

            let mut buf = [0; 4096];

            while let Ok(n) = rx.read(&mut buf).await {
                if n == 0 {
                    break;
                }

                let msg = std::str::from_utf8(&buf[..n]).unwrap();
                println!("Received {}", msg);

                if let Err(e) = tx.write_all(&buf[..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
        time::sleep(Duration::from_secs(2)).await;
    }
}