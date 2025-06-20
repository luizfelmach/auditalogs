use crate::{
    channel::{RxChannel, TxChannel},
    client::{elastic::ElasticClient, ethereum::EthereumClient},
    config::AppConfig,
    prometheus::Prometheus,
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub tx: TxChannel,
    pub rx: RxChannel,
    pub elastic: ElasticClient,
    pub ethereum: EthereumClient,
    pub prometheus: Prometheus,
}
