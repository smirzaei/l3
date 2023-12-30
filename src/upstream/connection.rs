use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct UpstreamConnection<S>
where
    S: AsyncWriteExt + AsyncReadExt + Unpin,
{
    stream: S,
}

impl<S> UpstreamConnection<S>
where
    S: AsyncReadExt + AsyncWriteExt + Unpin,
{
    pub async fn connect(host: &str) -> Result<UpstreamConnection<S>, io::Error> {
        panic!("not")
    }
}
