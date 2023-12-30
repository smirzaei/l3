use std::{
    io::{self},
    sync::Arc,
};

use tokio::io::{AsyncReadExt, AsyncWrite};
use tracing::{info, warn};

use crate::config::Config;

pub struct Client<S>
where
    S: AsyncReadExt + AsyncWrite + Unpin,
{
    conf: Arc<Config>,
    stream: S,
}

impl<S> Client<S>
where
    S: AsyncReadExt + AsyncWrite + Unpin,
{
    pub fn new(stream: S, conf: Arc<Config>) -> Client<S> {
        Client { stream, conf }
    }

    pub async fn serve(&mut self) -> io::Result<()> {
        let mut buffer: Vec<u8> = vec![0; self.conf.service.max_message_length];
        let mut n: usize;

        loop {
            // n = socket.read_exact(&mut buffer[0..4]).await?;
            // let payload_size = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
            let payload_size = self.stream.read_u32_le().await? as usize;
            info!(payload_size);
            if payload_size > self.conf.service.max_message_length {
                warn!(
                    payload_size,
                    self.conf.service.max_message_length,
                    "payload size is greater than the maximum"
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
