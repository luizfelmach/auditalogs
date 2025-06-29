use crate::{state::AppState, storage::Storage};
use std::sync::Arc;
use tracing::{debug, error, info, trace};

pub async fn storage(state: Arc<AppState>) {
    let config = state.config.clone();
    let rx = state.rx.clone();
    let elastic = config.elastic;
    let client = state.storage.clone();

    let mut buffer = Vec::new();

    info!("storage worker started with disable: {}", elastic.disable);

    while let Some(msg) = rx.storage.lock().await.recv().await {
        state.prometheus.elastic_queue.dec();
        trace!("received message for elastic");
        if elastic.disable {
            debug!("elastic disabled, skipping message");
            continue;
        }

        buffer.push(msg.clone());

        if buffer.len() >= 10_000 {
            if let Err(err) = client.store(&buffer).await {
                error!(?err, "error storing documents");
            }
            buffer.clear();
        }
    }
}
