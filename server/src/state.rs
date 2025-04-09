use std::process;
use tracing::error;

use crate::{
    channel::{self, RxChannel, TxChannel},
    client::{elastic::ElasticClient, ethereum::EthereumClient},
    config::AppConfig,
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub tx: TxChannel,
    pub rx: RxChannel,
    pub elastic: ElasticClient,
    pub ethereum: EthereumClient,
}

impl AppState {
    pub fn new() -> Self {
        let config = AppConfig::load("Config.toml");

        let (tx, rx) = channel::new(config.queue_size);

        let result = ElasticClient::new(
            config.elastic.url.clone(),
            config.elastic.username.clone(),
            config.elastic.password.clone(),
        );

        let Ok(elastic) = result else {
            error!("Could not create elastic client");
            process::exit(1);
        };

        let ethereum = EthereumClient::new(
            config.ethereum.url.clone(),
            config.ethereum.contract.clone(),
            config.ethereum.private_key.clone(),
        );

        let Ok(ethereum) = ethereum else {
            error!("Could not create ethereum client");
            process::exit(1);
        };

        return Self {
            config,
            tx,
            rx,
            elastic,
            ethereum,
        };
    }
}
