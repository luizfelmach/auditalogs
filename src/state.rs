use crate::{
    channel::{RxChannel, TxChannel},
    client::ethereum::EthereumClient,
    config::AppConfig,
    prometheus::Prometheus,
    storage::elasticsearch::ElasticsearchAdapter,
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub tx: TxChannel,
    pub rx: RxChannel,
    pub storage: ElasticsearchAdapter,
    pub ethereum: EthereumClient,
    pub prometheus: Prometheus,
}
