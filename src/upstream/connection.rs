use std::io;

use tokio::net::TcpStream;

use super::pool::Request;

pub struct Connection {
    address: &'static String,
    buffer_size: usize,
    stream: TcpStream,
    queue: async_channel::Receiver<Request>,
}

impl Connection {
    pub async fn connect(
        address: &'static String,
        buffer_size: usize,
        queue: async_channel::Receiver<Request>,
    ) -> io::Result<Self> {
        let stream = TcpStream::connect(&address).await?;

        Ok(Connection {
            address,
            buffer_size,
            stream,
            queue,
        })
    }

    pub async fn serve(&self) -> io::Result<()> {
        todo!()
    }
}
