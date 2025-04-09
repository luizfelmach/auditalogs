use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub queue_size: usize,
    pub batch_size: usize,
    pub ethereum_batch_size: usize,
    pub threads: usize,
    pub ethereum: EthereumConfig,
    pub elastic: ElasticConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EthereumConfig {
    pub url: String,
    pub contract: String,
    pub private_key: String,
    pub disable: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ElasticConfig {
    pub url: String,
    pub username: String,
    pub password: String,
    pub disable: bool,
}

impl AppConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file =
            File::open(&path).map_err(|e| format!("Failed to open config file: {}", e))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let config = toml::from_str(&contents)
            .map_err(|err| format!("Failed to parse TOML config: {}", err))?;

        Ok(config)
    }

    pub fn load(path: &str) -> AppConfig {
        match AppConfig::from_file(path) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Error reading config file: {err}");
                std::process::exit(1);
            }
        }
    }
}
