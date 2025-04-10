use std::sync::Arc;

use crate::{
    channel::{ElasticChannelItem, EthereumChannelItem},
    state::AppState,
    utils::{elastic_index, fingerprint},
};
use tracing::{debug, error, info, trace};

pub async fn worker(state: Arc<AppState>) {
    let config = state.config.clone();
    let rx = state.rx.clone();
    let tx = state.tx.clone();
    let mut counter = 0;
    let mut hash = String::new();
    let mut index = elastic_index(&config.name);

    info!("worker started with batch_size: {}", config.batch_size);
    debug!(counter = counter, hash = hash, index = index, "state");

    while let Some(msg) = rx.worker.lock().await.recv().await {
        trace!(?msg, "received message for processing");

        hash = fingerprint(&hash, &msg);
        counter += 1;

        debug!(counter = counter, hash = hash, index = index, "state");

        let item = ElasticChannelItem::new(index.clone(), msg.clone());
        if let Err(err) = tx.elastic.send(item).await {
            error!("failed to send message to elastic channel: {:?}", err);
        }

        if counter >= config.batch_size {
            let item = EthereumChannelItem::new(index.clone(), hash.clone().parse().unwrap());
            if let Err(err) = tx.ethereum.send(item).await {
                error!("failed to send message to ethereum channel: {:?}", err);
            }

            info!(
                "batch processing completed. items processed: {}, index: {} ({})",
                counter, index, hash
            );

            counter = 0;
            hash.clear();
            index = elastic_index(&config.name);
        }
    }
}
