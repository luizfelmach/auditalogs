use crate::state::AppState;
use std::sync::Arc;
use tracing::{debug, error, info, trace, warn};

pub async fn ethereum(state: Arc<AppState>) {
    let config = state.config.clone();
    let rx = state.rx.clone();
    let ethereum = config.ethereum;
    let client = state.ethereum.clone();

    let mut buffer = Vec::new();

    info!(
        "ethereum worker started with disable: {}, batch_size: {}",
        ethereum.disable, config.ethereum_batch_size
    );

    while let Some(msg) = rx.ethereum.lock().await.recv().await {
        state.prometheus.ethereum_queue.dec();
        trace!(hash = ?msg.hash, "received message for ethereum");

        if ethereum.disable {
            debug!("ethereum disabled, skipping message");
            continue;
        }

        buffer.push(msg);

        if buffer.len() >= config.ethereum_batch_size {
            debug!(batch_size = buffer.len(), "processing ethereum batch");

            let mut nonce = match client.nonce().await {
                Ok(n) => {
                    debug!(nonce = n, "fetched current nonce");
                    n
                }
                Err(err) => {
                    error!(error = ?err, "failed to fetch nonce");
                    continue;
                }
            };

            let mut txs = Vec::new();

            for content in buffer.iter() {
                trace!(current_nonce = nonce,  hash = ?content.hash,
                       "sending transaction");
                match client
                    .store(nonce, &content.id, content.hash.as_bytes().into())
                    .await
                {
                    Ok(tx_hash) => txs.push((nonce, tx_hash)),
                    Err(err) => {
                        error!(nonce = nonce, error = ?err, "failed to send tx");
                    }
                }
                nonce += 1;
            }

            debug!(
                tx_count = txs.len(),
                "waiting for transaction confirmations"
            );
            for (tx_nonce, tx_hash) in &txs {
                trace!(tx_nonce = tx_nonce, tx_hash = ?tx_hash, "waiting for tx confirmation");
                match client.wait_tx(*tx_hash).await {
                    Ok(_) => (),
                    Err(err) => {
                        warn!(tx_nonce = tx_nonce, tx_hash = ?tx_hash, error = ?err, "tx failed");
                        trace!(tx_nonce = tx_nonce, "attempting to remove failed tx");
                        if let Err(e) = client.remove_tx(*tx_nonce).await {
                            error!(tx_nonce = tx_nonce, error = ?e, "failed to remove tx");
                        }
                    }
                }
            }

            info!(tx_count = txs.len(), "ethereum batch processing completed");
            buffer.clear();
        }
    }
}
