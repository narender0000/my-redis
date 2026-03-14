use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6143").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buff = vec![0; 124];
            loop {
                match socket.read(&mut buff).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buff[..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });
    }
}

//If a variable is used before and after .await,
// Rust must store it inside the future's state machine.
// stack arrays would make each task very large ->[1024 bytes buffer]
// using Vec keeps the task small and scalable. ->Vec { ptr, len, cap }
