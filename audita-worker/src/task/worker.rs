use crate::{
    channel::{ElasticChannelItem, EthereumChannelItem, RxChannel, TxChannel},
    utils::{elastic_index, fingerprint},
};

const BATCH_SIZE: usize = 5;
const WORKER_NAME: &str = "worker";

pub async fn worker(tx: TxChannel, rx: RxChannel) {
    let mut state = BatchState::new();

    while let Some(msg) = rx.worker.lock().await.recv().await {
        state.inc(&msg);

        let item = ElasticChannelItem::new(state.index.clone(), msg.clone());
        if let Err(err) = tx.elastic.send(item).await {
            eprintln!("Failed to enqueue message to elastic: {err}");
        }

        if state.counter >= BATCH_SIZE {
            let item =
                EthereumChannelItem::new(state.index.clone(), state.hash.clone().parse().unwrap());
            if let Err(err) = tx.ethereum.send(item).await {
                eprintln!("Failed to enqueue message to ethereum: {err}")
            }
            state.reset();
        }
    }
}

struct BatchState {
    counter: usize,
    hash: String,
    index: String,
}

impl BatchState {
    fn new() -> Self {
        Self {
            counter: 0,
            hash: String::new(),
            index: elastic_index(&WORKER_NAME.into()),
        }
    }

    fn reset(&mut self) {
        self.counter = 0;
        self.hash.clear();
        self.index = elastic_index(&WORKER_NAME.into());
    }

    fn inc(&mut self, new: &String) {
        self.hash = fingerprint(&self.hash, new);
        self.counter += 1;
    }
}
