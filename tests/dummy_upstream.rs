use std::io::{self};

use l3::frame::Frame;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};
use tracing::{debug, error, info};

pub struct Server {
    pub port: u16,
    listener: TcpListener,
}

impl Server {
    pub async fn listen() -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        Ok(Server {
            port: addr.port(),
            listener,
        })
    }

    pub async fn serve(&mut self) -> io::Result<()> {
        loop {
            let (stream, _) = self
                .listener
                .accept()
                .await
                .expect("upstream accept failure");

            tokio::spawn(async move {
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
    let mut reader = BufReader::with_capacity(128, stream_reader);
    let mut buf = String::with_capacity(128);
    loop {
        match reader.read_line(&mut buf).await {
            Ok(0) => {
                info!("connection closed");
                break;
            }
            Ok(n) => {
                let rev = buf.chars().rev().collect::<String>();
                let frame = Frame::new(1, rev.len() as u32);
                let payload = [&frame.as_bytes(), rev.as_bytes()].concat();
                debug!(n, received = buf, reversed = rev);

                stream_writer.write_all(&payload).await?;
                buf.clear();
            }
            Err(e) => {
                error!(err = ?e, "upstream read failure");
                break;
            }
        }
    }

    Ok(())
}
