use std::io;

use l3::frame::Frame;
use rand::{distributions::DistString, Rng};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::info;

pub struct Client<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    addr: String,
    stream: T,
    client_id: usize,
}

impl Client<TcpStream> {
    pub async fn connect(client_id: usize, addr: String) -> io::Result<Self> {
        let stream = TcpStream::connect(&addr).await?;
        let client = Client {
            addr,
            stream,
            client_id,
        };

        Ok(client)
    }

    pub async fn send_request(&mut self, req_id: usize, print_result: bool) -> io::Result<()> {
        let (mut stream_reader, mut stream_writer) = self.stream.split();

        let char_len = rand::thread_rng().gen_range(3..4);
        let mut msg: String =
            rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), char_len);
        msg.push('\n');

        let msg_bytes = msg.as_bytes();
        let frame = Frame::new(1, msg_bytes.len().try_into().unwrap());
        let payload = [&frame.as_bytes(), msg_bytes].concat();

        stream_writer.write_all(&payload).await?;

        let mut buf: Vec<u8> = vec![0; msg_bytes.len()];

        stream_reader.read_exact(&mut buf).await?;
        let reversed = std::str::from_utf8(&buf).unwrap();
        let expected = msg.chars().rev().collect::<String>();

        if print_result {
            info!(
                client_id = self.client_id,
                req_id,
                original = msg,
                expected,
                result = reversed
            )
        }

        assert_eq!(expected, reversed);
        Ok(())
    }
}
