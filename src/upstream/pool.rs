use std::{future::Future, io};

use crate::config::Config;

pub trait AsyncRequestQueue {
    fn queue_request(
        &self,
        buff: &mut Vec<u8>,
        message_length: usize,
    ) -> impl Future<Output = Result<usize, io::Error>> + Send;
}

pub struct Pool {
    config: &'static Config,
}

impl Pool {
    pub fn new(config: &'static Config) -> Self {
        Pool { config }
    }
}

impl AsyncRequestQueue for Pool {
    async fn queue_request(
        &self,
        buff: &mut Vec<u8>,
        message_length: usize,
    ) -> Result<usize, io::Error> {
        todo!()
    }
}
