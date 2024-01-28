use std::io;

use tokio::{io::AsyncReadExt, net::TcpStream};
use tracing::info;

use crate::{
    config::Config,
    downstream::server::Server,
    upstream::{self, pool::Pool},
};

pub struct Daemon {
    conf: &'static Config,
    upstream_pool: &'static Pool,
    downstream_server: &'static Server<Pool>,
}

impl Daemon {
    pub fn new(conf: &'static mut Config) -> Self {
        info!("instantiating daemon");
        let upstream_pool: &'static mut Pool = Box::leak(Box::new(Pool::new(conf)));
        let downstream_server: &'static mut Server<_> =
            Box::leak(Box::new(Server::new(conf, upstream_pool)));

        Daemon {
            conf,
            upstream_pool,
            downstream_server,
        }
    }

    pub async fn run(&self) -> io::Result<()> {
        info!("running the daemon");
        // TODO: need to handle graceful shutdowns

        self.upstream_pool.start();
        self.downstream_server.start().await?;

        Ok(())
    }
}
