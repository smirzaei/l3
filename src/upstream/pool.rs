use std::{io, sync::Arc};

use tokio::sync::Mutex;
use tracing::info;

use crate::config::Config;

pub struct Request {
    buffer: Vec<u8>,
    message_length: usize,
}

pub struct Pool {
    config: Arc<Config>,
    queue: Arc<Mutex<Vec<Request>>>, // Should we use the standard mutex?
}

pub trait AsyncRequestQueue {
    async fn queue_request(&self, req: &Request) -> io::Result<()>;
}

impl Pool {
    pub fn new(conf: Arc<Config>) -> Pool {
        info!("🏊‍♂️ initializing the pool");
        for host in &conf.upstream.hosts {
            for i in 0..conf.upstream.connections {
                todo!()
            }
        }

        Pool {
            config: conf,
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl AsyncRequestQueue for Pool {
    async fn queue_request(&self, req: &Request) -> io::Result<()> {
        todo!()
    }
}
