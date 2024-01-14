use std::{
    io::{self},
    sync::{Arc, PoisonError},
};

use tokio::{
    io::{AsyncReadExt, AsyncWrite, AsyncWriteExt},
    sync::Mutex,
};
use tracing::{info, warn};

use crate::{config::Config, upstream::pool::AsyncRequestQueue};

pub struct Client<T, U>
where
    T: AsyncReadExt,
    U: AsyncRequestQueue + 'static,
{
    conf: &'static Config,
    stream: T,
    queue: &'static U,
}

impl<T, U> Client<T, U>
where
    T: AsyncReadExt + AsyncWrite + Unpin,
    U: AsyncRequestQueue,
{
    pub fn new(stream: T, conf: &'static Config, queue: &'static U) -> Self {
        Client {
            stream,
            conf,
            queue,
        }
    }

    pub async fn serve(&mut self) -> io::Result<()> {
        let b: Vec<u8> = vec![0; self.conf.service.max_message_length];
        let buffer = Arc::new(Mutex::new(b));

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

            let downstream_buff = buffer.clone();
            let upstream_buff = buffer.clone();

            let mut downstream_mutex = downstream_buff.lock().await;
            let buffer: &mut Vec<u8> = downstream_mutex.as_mut();
            n = self.stream.read_exact(&mut buffer[0..payload_size]).await?;
            info!(n, a = format!("{buffer:?}"));
            drop(downstream_mutex);

            n = self.queue.queue_request(upstream_buff, n).await?;

            let mut downstream_mutex = downstream_buff.lock().await;
            let buffer: &mut Vec<u8> = downstream_mutex.as_mut();
            self.stream.write_all(&buffer[0..n]).await?;
        }
    }
}
