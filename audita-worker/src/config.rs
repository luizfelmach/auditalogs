use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub queue_size: usize,
    pub threads: usize,
    pub ethereum: EthereumConfig,
    pub elastic: ElasticConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EthereumConfig {
    pub url: String,
    pub contract: String,
    pub primary_key: String,
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
    pub fn load() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name("config").required(false))
            .add_source(Environment::default().separator("_"))
            .build()?
            .try_deserialize()
    }
}
