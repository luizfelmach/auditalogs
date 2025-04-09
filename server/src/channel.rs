use alloy::primitives::FixedBytes;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub type WorkerChannelItem = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthereumChannelItem {
    pub index: String,
    pub hash: FixedBytes<32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElasticChannelItem {
    pub index: String,
    pub content: String,
}

impl EthereumChannelItem {
    pub fn new(index: String, hash: FixedBytes<32>) -> Self {
        Self { index, hash }
    }
}

impl ElasticChannelItem {
    pub fn new(index: String, content: String) -> Self {
        Self { index, content }
    }
}

#[derive(Clone)]
pub struct TxChannel {
    pub worker: Arc<mpsc::Sender<WorkerChannelItem>>,
    pub ethereum: Arc<mpsc::Sender<EthereumChannelItem>>,
    pub elastic: Arc<mpsc::Sender<ElasticChannelItem>>,
}

#[derive(Clone)]
pub struct RxChannel {
    pub worker: Arc<Mutex<mpsc::Receiver<WorkerChannelItem>>>,
    pub ethereum: Arc<Mutex<mpsc::Receiver<EthereumChannelItem>>>,
    pub elastic: Arc<Mutex<mpsc::Receiver<ElasticChannelItem>>>,
}

pub fn new(channel_size: usize) -> (TxChannel, RxChannel) {
    let (worker_tx, worker_rx) = mpsc::channel(channel_size);
    let (ethereum_tx, ethereum_rx) = mpsc::channel(channel_size);
    let (elastic_tx, elastic_rx) = mpsc::channel(channel_size);

    let shared = TxChannel {
        worker: Arc::new(worker_tx),
        ethereum: Arc::new(ethereum_tx),
        elastic: Arc::new(elastic_tx),
    };

    let receivers = RxChannel {
        worker: Arc::new(Mutex::new(worker_rx)),
        ethereum: Arc::new(Mutex::new(ethereum_rx)),
        elastic: Arc::new(Mutex::new(elastic_rx)),
    };

    return (shared, receivers);
}
