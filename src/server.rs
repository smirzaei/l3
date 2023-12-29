use std::io;
use std::io::ErrorKind;
use std::sync::Arc;

use tracing::error;
use tracing::info;

use crate::client;
use crate::config;

pub struct Server {
    conf: Arc<config::Config>,
}

impl Server {
    pub fn new(conf: Arc<config::Config>) -> Server {
        Server { conf }
    }

    pub async fn start(&self) -> io::Result<()> {
        let address = format!("{}:{}", self.conf.service.host, self.conf.service.port);
        let listener = tokio::net::TcpListener::bind(&address).await?;
        info!(address, "👂listening for new connections");

        loop {
            match listener.accept().await {
                Err(e) => {
                    error!(err = e.to_string(), "error accepting a connection")
                }
                Ok((socket, addr)) => {
                    tokio::spawn(async move {
                        info!(address = addr.to_string(), "new connection");
                        let mut conn = client::Client::new(socket);
                        match &conn.serve().await {
                            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                                info!(address = addr.to_string(), "client disconnected");
                            }
                            _ => {}
                        }
                    });
                }
            }
        }
    }
}
