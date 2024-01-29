use std::{cmp::min, future::Future, io, sync::Arc, time::Duration};

use tokio::sync::{oneshot, Mutex};
use tracing::{debug, error, info, warn};

use crate::config::Config;

use super::connection::Connection;

pub struct Request {
    pub(super) buff: Arc<Mutex<Vec<u8>>>,
    pub(super) msg_len: usize,
    pub(super) done: oneshot::Sender<i64>,
}

pub trait AsyncRequestQueue {
    fn queue_request(
        &self,
        buff: Arc<Mutex<Vec<u8>>>,
        msg_len: usize,
    ) -> impl Future<Output = Result<usize, io::Error>> + Send;
}

pub struct Pool {
    config: &'static Config,
    queue_tx: async_channel::Sender<Request>,
    queue_rx: async_channel::Receiver<Request>,
}

impl Pool {
    // TODO: For consistency's sake either all new functions should be async or
    //  create an initializer method such as start.
    pub fn new(config: &'static Config) -> Self {
        let (tx, rx) = async_channel::unbounded::<Request>();

        Pool {
            config,
            queue_tx: tx,
            queue_rx: rx,
        }
    }

    pub fn start(&'static self) {
        info!("starting the upstream pool");
        for address in &self.config.upstream.hosts {
            info!(
                address,
                connections = self.config.upstream.connections,
                "establishing connection(s)"
            );

            for _ in 0..self.config.upstream.connections {
                self.handle_connection(address);
            }
        }

        // TODO: wait for at least one active connection before considering the pool as ready
    }

    fn handle_connection(&'static self, address: &'static String) {
        let mut try_num = 0;

        tokio::spawn(async move {
            loop {
                let rx = self.queue_rx.clone();
                // TODO: These error branches need to handle graceful shutdowns
                //  if we are shutting down the application and something is waiting in the Err branch
                //  it should cancel.
                match Connection::connect(address, self.config.service.max_message_length, rx).await
                {
                    Err(e) => {
                        try_num += 1;
                        let sleep_duration = Duration::from_secs(min(60, try_num));
                        error!(try_num, address, err = ?e, ?sleep_duration, "failed to connect to upstream");
                        tokio::time::sleep(sleep_duration).await;
                        continue;
                    }
                    Ok(ref mut c) => {
                        // reset the try num since the connection was successful
                        try_num = 0;
                        match c.serve().await {
                            Ok(_) => {
                                // Nothing to do here. The connection was
                                // terminated as planned and we are not going to
                                // reconnect
                                return;
                            }
                            Err(e) => {
                                error!(address, err = ?e, "upstream connection failure. Reconnecting.");
                                continue;
                            }
                        }
                    }
                }
            }
        });
    }
}

impl AsyncRequestQueue for Pool {
    async fn queue_request(
        &self,
        buf: Arc<Mutex<Vec<u8>>>,
        message_len: usize,
    ) -> Result<usize, io::Error> {
        let (tx, rx) = oneshot::channel::<i64>();
        let req = Request {
            buff: buf,
            msg_len: message_len,
            done: tx,
        };

        self.queue_tx.send(req).await;
        match rx.await {
            Err(_e) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "connection was interrupted",
            )),
            Ok(n) if n < 0 => Err(io::Error::new(io::ErrorKind::Other, "upstream error")),
            Ok(n) => Ok(usize::try_from(n).unwrap()),
        }
    }
}
