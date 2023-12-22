use std::io;

use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await?;
    info!(address = addr, "🚀 starting the application");

    loop {
        match listener.accept().await {
            Err(e) => {
                error!(err = e.to_string(), "error accepting a connection")
            }
            Ok((socket, addr)) => {
                tokio::spawn(async move {
                    info!(address = addr.to_string(), "new connection");
                    handle_connection(socket).await;
                });
            }
        }
    }
}

async fn handle_connection(mut socket: TcpStream) -> io::Result<()> {
    const MAX_PAYLOAD_SIZE: usize = 32;
    let mut buffer: [u8; MAX_PAYLOAD_SIZE] = [0; MAX_PAYLOAD_SIZE];
    let mut n: usize;

    loop {
        // n = socket.read_exact(&mut buffer[0..4]).await?;
        // let payload_size = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
        let payload_size = socket.read_u32_le().await? as usize;
        info!(payload_size);
        if payload_size > MAX_PAYLOAD_SIZE {
            warn!(
                payload_size,
                MAX_PAYLOAD_SIZE, "payload size is greater than the maximum"
            );

            return Err(io::Error::new(
                io::ErrorKind::Other,
                "payload size is greater than the maximum",
            ));
        }

        n = socket.read_exact(&mut buffer[0..payload_size]).await?;
        info!(n, a = format!("{buffer:?}"));
    }
}
