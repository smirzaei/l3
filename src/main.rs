use std::error::Error;

use l3::{config::Config, daemon::Daemon};
use tracing::info;

mod cli;
mod config;
mod daemon;
mod downstream;
mod frame;
mod upstream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let args = cli::parse_args();
    info!("🚀 Starting the application");

    let conf: &'static mut Config = Box::leak(Box::new(Config::read_from_file(&args.config)?));
    info!(config = ?conf, "⚙️ loaded configuration");

    let daemon = Daemon::new(conf);
    daemon.run().await?;

    Ok(())
}
