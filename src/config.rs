use serde::Deserialize;
use std::{error::Error, fs};
use tracing::info;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub service: Service,
    pub upstream: Upstream,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Service {
    pub host: String,
    pub port: u16,

    #[serde(with = "serde_humanize_rs")]
    pub max_message_length: usize,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use std::{error::Error, path::PathBuf};

    use super::Config;

    #[test]
    fn properly_deserilizes_the_config() -> Result<(), Box<dyn Error>> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("config/config.toml");
        let conf_path = d.into_os_string().into_string().expect("conf path faliure");
        let conf = Config::new(&conf_path)?;

        let expected = Config {
            service: super::Service {
                host: String::from("0.0.0.0"),
                port: 8000,
                max_message_length: 32,
            },
            upstream: super::Upstream {
                hosts: vec![
                    String::from("127.0.0.1:4444"),
                    String::from("127.0.0.1:4445"),
                ],
            },
        };

        assert_eq!(expected, conf);

        Ok(())
    }
}
