use crate::entity::{Document, Fingerprint, Storable};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

#[derive(Clone)]
pub struct TxChannel {
    pub worker: Arc<mpsc::Sender<Document>>,
    pub ethereum: Arc<mpsc::Sender<Fingerprint>>,
    pub storage: Arc<mpsc::Sender<Storable>>,
}

#[derive(Clone)]
pub struct RxChannel {
    pub worker: Arc<Mutex<mpsc::Receiver<Document>>>,
    pub ethereum: Arc<Mutex<mpsc::Receiver<Fingerprint>>>,
    pub storage: Arc<Mutex<mpsc::Receiver<Storable>>>,
}

pub fn new(channel_size: usize) -> (TxChannel, RxChannel) {
    let (worker_tx, worker_rx) = mpsc::channel(channel_size);
    let (ethereum_tx, ethereum_rx) = mpsc::channel(channel_size);
    let (elastic_tx, elastic_rx) = mpsc::channel(channel_size);

    let shared = TxChannel {
        worker: Arc::new(worker_tx),
        ethereum: Arc::new(ethereum_tx),
        storage: Arc::new(elastic_tx),
    };

    let receivers = RxChannel {
        worker: Arc::new(Mutex::new(worker_rx)),
        ethereum: Arc::new(Mutex::new(ethereum_rx)),
        storage: Arc::new(Mutex::new(elastic_rx)),
    };

    return (shared, receivers);
}
