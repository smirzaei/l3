use std::error::Error;

use l3::{config::Config, daemon::Daemon};
use tracing::info;

mod cli;
pub mod config;
pub mod daemon;
mod downstream;
pub mod frame;
pub mod upstream;

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
