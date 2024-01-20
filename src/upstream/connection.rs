use std::io;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    select,
};
use tracing::warn;

use crate::frame::Frame;

use super::pool::Request;

pub struct Connection<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    address: &'static String,
    buffer_size: usize,
    stream: T,
    queue: async_channel::Receiver<Request>,
}

impl Connection<TcpStream> {
    pub async fn connect(
        address: &'static String,
        buffer_size: usize,
        queue: async_channel::Receiver<Request>,
    ) -> io::Result<Self> {
        let stream = TcpStream::connect(&address).await?;
        let con = Connection {
            address,
            buffer_size,
            stream,
            queue,
        };

        Ok(con)
    }
}

impl<T> Connection<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    pub async fn serve(&mut self) -> io::Result<()> {
        loop {
            select! {
                x = self.queue.recv() => match x {
                    Err(e) => {
                        // TODO: Does this error happen only if the channel is closed? Overall need better error handling here...
                        warn!(err = ?e, addr=self.address, "queue receive failure");
                        return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
                     },
                    Ok(req) => {
                        let mut mut_guard = req.buff.lock().await;
                        let buff: &mut Vec<u8> = mut_guard.as_mut();

                        if let Err(e) = self.stream.write_all(&buff[0..req.msg_len]).await {
                            // Err here means that the receiver is already deallocated
                            let _ = req.done.send(-1);
                            return Err(e);
                        }

                        if let Err(e) = self.stream.read_exact(&mut buff[0..8]).await {
                            // Err here means that the receiver is already deallocated
                            let _ = req.done.send(-1);
                            return Err(e);
                        }

                        let frame = match Frame::from_bytes(
                            &buff[0..8]
                            .try_into()
                            .expect("couldn't convert buffer into [u8;8]")
                        ) {
                            Ok(f) => f,
                            Err(e) => {
                                // TODO: need to handle the FrameError
                                // Err here means that the receiver is already deallocated
                                let _ = req.done.send(-1);
                                return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
                            },
                        };

                        if frame.msg_len as usize > self.buffer_size {
                            warn!(
                                frame.msg_len,
                                self.buffer_size,
                                "payload size is greater than the maximum"
                            );

                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                "payload size is greater than the maximum",
                            ));
                        }

                        if let Err(e) = self.stream.read_exact(&mut buff[0..frame.msg_len as usize]).await {
                            // Err here means that the receiver is already deallocated
                            let _ = req.done.send(-1);
                            return Err(e);
                        }

                        // Err here means that the receiver is already deallocated
                        let _ = req.done.send(frame.msg_len as i64); // u32 can fit in an i64

                        // mut_guard unlock happens here
                    },
                }
            }
            // TODO: add a branch for cancellation
        }
    }
}
