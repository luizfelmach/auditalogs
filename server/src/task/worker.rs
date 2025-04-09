use std::sync::Arc;

use crate::{
    channel::{ElasticChannelItem, EthereumChannelItem},
    state::AppState,
    utils::{elastic_index, fingerprint},
};
use tracing::{error, info};

pub async fn worker(state: Arc<AppState>) {
    let config = state.config.clone();
    let rx = state.rx.clone();
    let tx = state.tx.clone();
    let mut counter = 0;
    let mut hash = String::new();
    let mut index = elastic_index(&config.name);

    while let Some(msg) = rx.worker.lock().await.recv().await {
        hash = fingerprint(&hash, &msg);
        counter += 1;

        let item = ElasticChannelItem::new(index.clone(), msg.clone());
        if let Err(err) = tx.elastic.send(item).await {
            error!("Failed to send message to elastic channel: {:?}", err);
        }

        if counter >= config.batch_size {
            let item = EthereumChannelItem::new(index.clone(), hash.clone().parse().unwrap());
            if let Err(err) = tx.ethereum.send(item).await {
                error!("Failed to send message to ethereum channel: {:?}", err);
            }
            counter = 0;
            hash.clear();
            index = elastic_index(&config.name);
        }
    }
    info!("Worker channel closed. Exiting worker task");
}
