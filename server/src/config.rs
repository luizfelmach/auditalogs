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
    pub fn load(path: String) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config = toml::from_str(&contents)?;

        Ok(config)
    }
}
