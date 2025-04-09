use crate::state::AppState;
use std::sync::Arc;
use tracing::{error, info, warn};

pub async fn ethereum(state: Arc<AppState>) {
    let config = state.config.clone();
    let rx = state.rx.clone();
    let ethereum = config.ethereum;
    let client = state.ethereum.clone();

    if ethereum.disable {
        warn!("Module is disabled. Skipping messages from channel");
    }

    let mut buffer = Vec::new();

    while let Some(msg) = rx.ethereum.lock().await.recv().await {
        if ethereum.disable {
            continue;
        }

        buffer.push(msg);

        if buffer.len() >= config.ethereum_batch_size {
            let mut nonce = match client.nonce().await {
                Ok(n) => n,
                Err(err) => {
                    error!("Failed to fetch nonce: {:?}", err);
                    continue;
                }
            };

            let mut txs = Vec::new();

            for content in buffer.iter() {
                match client.send_tx(nonce, &content.index, content.hash, 3).await {
                    Ok(tx_hash) => txs.push((nonce, tx_hash)),
                    Err(err) => {
                        error!("Failed to send tx (nonce {}): {:?}", nonce, err);
                    }
                }
                nonce += 1;
            }

            for (tx_nonce, tx_hash) in &txs {
                match client.wait_for_tx(*tx_hash).await {
                    Ok(_) => (),
                    Err(err) => {
                        warn!("Tx ({}) {} failed: {:?}", tx_nonce, tx_hash, err);
                        if let Err(e) = client.remove_tx(*tx_nonce).await {
                            error!("Failed to remove tx {}: {:?}", tx_nonce, e);
                        }
                    }
                }
            }

            buffer.clear();
        }
    }
    info!("Ethereum channel closed. Exiting ethereum task");
}
