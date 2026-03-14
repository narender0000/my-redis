use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let (mut rd, mut wr) = io::split(socket);

        // Write data in the background
        tokio::spawn(async move {
            wr.write_all(b"hello\\n").await?;
            wr.write_all(b"world\r\n").await?;

            // Sometimes, the rust type inferencer needs
            // a little help
            Ok::<_, io::Error>(())
        });

        let mut buf = vec![0; 128];

        loop {
            let n = rd.read(&mut buf).await?;

            if n == 0 {
                break;
            }

            println!("GOT {:?}", &buf[..n]);
        }
    }
    Ok(())
}
