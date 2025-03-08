use serde::Deserialize;
use std::{fmt, fs};

pub fn parse(file: String) -> Config {
    let config_str = fs::read_to_string(file).unwrap_or_default();
    let config = toml::from_str(&config_str).unwrap();
    return config;
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_name")]
    pub name: String,

    #[serde(default = "default_batch")]
    pub batch: usize,

    #[serde(default = "default_queue_worker")]
    pub queue_worker: usize,

    #[serde(default = "default_queue_ethereum")]
    pub queue_ethereum: usize,

    #[serde(default = "default_queue_elastic")]
    pub queue_elastic: usize,

    #[serde(default = "default_dispatchers")]
    pub dispatchers: usize,

    #[serde(default = "default_elastic")]
    pub elastic: Elastic,

    #[serde(default = "default_ethereum")]
    pub ethereum: Ethereum,
}

#[derive(Debug, Deserialize)]
pub struct Elastic {
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Ethereum {
    pub url: String,
    pub contract: String,
    pub primary_key: String,
}

impl fmt::Display for Elastic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Elastic:\n  - URL: {}\n  - Username: {}\n  - Password: ****",
            self.url, self.username
        )
    }
}

// Implementação de Display para Ethereum
impl fmt::Display for Ethereum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ethereum:\n  - URL: {}\n  - Contract: {}\n  - Primary Key: ****",
            self.url, self.contract
        )
    }
}

// Implementação de Display para Config
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n============= CONFIG =============\n\
           App:\n\
           - Name: {}\n\
           - Port: {}\n\
           - Batch: {}\n\
           \nChannel:\n\
           - Queue Workers: {}\n\
           - Queue Ethereum: {}\n\
           - Queue Elastic: {}\n\
           - Dispatchers: {}\n\
           \n{}\n\n{}\n===================================",
            self.name,
            self.port,
            self.batch,
            self.queue_worker,
            self.queue_ethereum,
            self.queue_elastic,
            self.dispatchers,
            self.elastic,
            self.ethereum
        )
    }
}

fn default_name() -> String {
    String::from("worker")
}

fn default_port() -> u16 {
    8080
}

fn default_batch() -> usize {
    10_000
}

fn default_queue_worker() -> usize {
    1024
}

fn default_queue_ethereum() -> usize {
    128
}

fn default_queue_elastic() -> usize {
    128
}

fn default_dispatchers() -> usize {
    2
}

fn default_elastic() -> Elastic {
    Elastic {
        url: "http://localhost:9200".into(),
        username: "elastic".into(),
        password: "changeme".into(),
    }
}

fn default_ethereum() -> Ethereum {
    Ethereum {
        url: "http://localhost:8545".into(),
        contract: "0x5FbDB2315678afecb367f032d93F642f64180aa3".into(),
        primary_key: "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".into(),
    }
}
