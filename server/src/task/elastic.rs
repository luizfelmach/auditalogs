use crate::state::AppState;
use std::sync::Arc;
use tracing::{debug, error, info, trace, warn};

pub async fn elastic(state: Arc<AppState>) {
    let config = state.config.clone();
    let rx = state.rx.clone();
    let elastic = config.elastic;
    let client = state.elastic.clone();

    info!("elastic worker started with disable: {}", elastic.disable);

    while let Some(msg) = rx.elastic.lock().await.recv().await {
        trace!(?msg.index, content_len = msg.content.len(), "received message for elastic");
        if elastic.disable {
            debug!("elastic disabled, skipping message");
            continue;
        }

        let value = match serde_json::from_str(&msg.content) {
            Ok(v) => v,
            Err(err) => {
                warn!(error = ?err, "failed to parse JSON");
                continue;
            }
        };

        debug!(index = msg.index, "storing document in elastic");

        if let Err(err) = client.store(&msg.index, &value).await {
            error!(error = ?err, index = msg.index, "failed to store document");
        } else {
            trace!(index = msg.index, "document successfully stored");
        }
    }
}
