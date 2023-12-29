use std::io::{self};

use tokio::io::{AsyncReadExt, AsyncWrite};
use tracing::{info, warn};

pub struct Client<S>
where
    S: AsyncReadExt + AsyncWrite + Unpin,
{
    stream: S,
}

impl<S> Client<S>
where
    S: AsyncReadExt + AsyncWrite + Unpin,
{
    pub fn new(stream: S) -> Client<S> {
        Client { stream }
    }

    pub async fn serve(&mut self) -> io::Result<()> {
        const MAX_PAYLOAD_SIZE: usize = 32;
        let mut buffer: [u8; MAX_PAYLOAD_SIZE] = [0; MAX_PAYLOAD_SIZE];
        let mut n: usize;

        loop {
            // n = socket.read_exact(&mut buffer[0..4]).await?;
            // let payload_size = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
            let payload_size = self.stream.read_u32_le().await? as usize;
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

            n = self.stream.read_exact(&mut buffer[0..payload_size]).await?;
            info!(n, a = format!("{buffer:?}"));
        }
    }
}
