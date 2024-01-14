use std::{error::Error, sync::Arc};

use crate::downstream::server::Server;
use config::Config;
use tracing::info;

use crate::upstream::pool::Pool;

mod cli;
mod config;
mod downstream;
mod upstream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let args = cli::parse_args();
    info!("🚀 Starting the application");

    let conf: &'static mut Config = Box::leak(Box::new(Config::read_from_file(&args.config)?));
    info!(config = ?conf, "⚙️ loaded configuration");

    let p = Pool::new(conf).await;
    let pool = Box::leak(Box::new(p));
    pool.start();

    let server: &'static mut Server<_> = Box::leak(Box::new(Server::new(conf, pool)));
    server.start().await?;

    Ok(())
}
