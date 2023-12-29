use serde::Deserialize;
use std::{error::Error, fs};
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub service: Service,
    pub upstream: Upstream,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Upstream {
    pub hosts: Vec<String>,
}

impl Config {
    pub fn new(conf_path: &str) -> Result<Config, Box<dyn Error>> {
        info!(path = conf_path, "👀 reading the config");
        let conf_data = fs::read_to_string(conf_path)?;
        let config: Config = toml::from_str(&conf_data)?;

        Ok(config)
    }
}
