use std::{
    io::{self, ErrorKind},
    sync::Arc,
};

use tracing::{error, info, warn};

use crate::{config::Config, downstream::client::Client, upstream::pool::AsyncRequestQueue};

pub struct Server<T>
where
    T: AsyncRequestQueue + Send + Sync + 'static,
{
    config: &'static Config,
    queue: &'static T,
}

impl<T> Server<T>
where
    T: AsyncRequestQueue + Send + Sync,
{
    pub fn new(config: &'static Config, queue: &'static T) -> Self {
        Server { config, queue }
    }

    pub async fn start(&'static self) -> io::Result<()> {
        info!("starting the downstream server");

        let listener = tokio::net::TcpListener::bind("localhost:8000").await?;
        loop {
            match listener.accept().await {
                Err(e) => {
                    error!(err = ?e, "error accepting a connection")
                }
                Ok((stream, addr)) => {
                    info!(?addr, "new connection");
                    tokio::spawn(async {
                        let mut c = Client::new(stream, self.config, self.queue);
                        match &c.serve().await {
                            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                                info!("client disconnected");
                            }
                            Err(e) => {
                                warn!(err = e.kind().to_string(), "client error");
                            }
                            Ok(()) => {}
                        }
                    });
                }
            }
        }
    }
}
