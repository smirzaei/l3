use std::io::{self, Read};

use l3::frame::Frame;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};
use tracing::error;

struct Server {
    port: u16,
    listener: TcpListener,
}

impl Server {
    async fn new() -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1").await?;
        let addr = listener.local_addr()?;

        Ok(Server {
            port: addr.port(),
            listener,
        })
    }

    async fn serve(&mut self) -> io::Result<()> {
        loop {
            let (stream, _) = self
                .listener
                .accept()
                .await
                .expect("upstream accept failure");

            let handle = tokio::spawn(async move {
                // TODO: Don't think that this will break the test...
                handle_connection(stream)
                    .await
                    .expect("handle connection failure");
            });
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let (stream_reader, mut stream_writer) = stream.split();
    let mut reader = BufReader::new(stream_reader);
    let mut buf = String::with_capacity(1024);
    loop {
        match reader.read_line(&mut buf).await {
            Ok(0) => {
                // Nothing more to read
                break;
            }
            Ok(n) => {
                let x = buf.chars().rev().collect::<String>();
                let frame = Frame::new(1, x.len() as u32);
                let payload = [&frame.as_bytes(), x.as_bytes()].concat();

                stream_writer.write_all(&payload).await?;
            }
            Err(e) => {
                error!(err = ?e, "upstream read failure");
                break;
            }
        }
    }

    Ok(())
}
