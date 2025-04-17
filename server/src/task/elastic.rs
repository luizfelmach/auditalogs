use crate::state::AppState;
use std::sync::Arc;
use tracing::{debug, error, info, trace};

pub async fn elastic(state: Arc<AppState>) {
    let config = state.config.clone();
    let rx = state.rx.clone();
    let elastic = config.elastic;
    let client = state.elastic.clone();

    let mut buffer = Vec::new();

    info!("elastic worker started with disable: {}", elastic.disable);

    while let Some(msg) = rx.elastic.lock().await.recv().await {
        state.prometheus.elastic_queue.dec();
        trace!(?msg.index, content_len = msg.content.len(), "received message for elastic");
        if elastic.disable {
            debug!("elastic disabled, skipping message");
            continue;
        }

        buffer.push(msg.clone());

        if buffer.len() >= 10_000 {
            if let Err(err) = client.store(buffer.clone()).await {
                error!(?err, "error storing documents");
            }
            buffer.clear();
        }
    }
}
