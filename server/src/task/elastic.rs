use crate::state::AppState;
use std::sync::Arc;
use tracing::{error, warn};

pub async fn elastic(state: Arc<AppState>) {
    let config = state.config.clone();
    let rx = state.rx.clone();
    let elastic = config.elastic;
    let client = state.elastic.clone();

    if elastic.disable {
        warn!("Module is disabled. Skipping messages from channel.");
    }

    while let Some(msg) = rx.elastic.lock().await.recv().await {
        if elastic.disable {
            continue;
        }

        let value = match serde_json::from_str(&msg.content) {
            Ok(v) => v,
            Err(err) => {
                warn!("Failed to parse JSON: {:?}", err);
                continue;
            }
        };

        if let Err(err) = client.store(&msg.index, &value).await {
            error!("Failed to store document: {:?}", err);
        }
    }
    warn!("Elastic channel closed. Exiting elastic task");
}
