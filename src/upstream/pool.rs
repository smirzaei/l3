use std::{future::Future, io, sync::Arc};

use tokio::sync::{oneshot, Mutex};

use crate::config::Config;

struct Request {
    buff: Arc<Mutex<Vec<u8>>>,
    message_len: usize,
    done: oneshot::Sender<isize>,
}

pub trait AsyncRequestQueue {
    fn queue_request(
        &self,
        buff: Arc<Mutex<Vec<u8>>>,
        message_len: usize,
    ) -> impl Future<Output = Result<usize, io::Error>> + Send;
}

pub struct Pool {
    config: &'static Config,
    queue_tx: async_channel::Sender<Request>,
    queue_rx: async_channel::Receiver<Request>,
}

impl Pool {
    pub fn new(config: &'static Config) -> Self {
        let (tx, rx) = async_channel::unbounded::<Request>();
        Pool {
            config,
            queue_tx: tx,
            queue_rx: rx,
        }
    }
}

impl AsyncRequestQueue for Pool {
    async fn queue_request(
        &self,
        buff: Arc<Mutex<Vec<u8>>>,
        message_len: usize,
    ) -> Result<usize, io::Error> {
        let (tx, rx) = oneshot::channel::<isize>();
        let req = Request {
            buff,
            message_len,
            done: tx,
        };

        self.queue_tx.send(req).await;

        todo!()
    }
}
