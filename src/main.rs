use std::{error::Error, sync::Arc};

use tracing::info;

mod cli;
mod client;
mod config;
mod server;
mod upstream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let args = cli::parse_args();
    info!(args = ?args, "🚀 starting the application");

    let config = Arc::new(config::Config::new(&args.config)?);
    info!(config = ?config, "⚙️ loaded configuration");

    let srv_cfg = config.clone();
    let server = server::Server::new(srv_cfg);
    let server_handle = tokio::spawn(async move {
        server.start().await.expect("failed to start the server");
    });

    server_handle.await?;
    Ok(())
}
