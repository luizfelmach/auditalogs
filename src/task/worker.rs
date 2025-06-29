use crate::{
    batch::Batch,
    entity::{Fingerprint, Storable},
    state::AppState,
};
use anyhow::Result;
use serde_json::{Map, Value};
use std::sync::Arc;
use tracing::{debug, error, info, trace, warn};

pub async fn worker(state: Arc<AppState>) {
    let mut batch = Batch::new();
    let config = &state.config;
    let rx = state.rx.clone();

    info!("worker started");

    while let Some(raw) = rx.worker.lock().await.recv().await {
        trace!(?raw, "received new document from queue");
        debug!(?batch);
        state.prometheus.logs_queue.dec();

        batch.add(&raw).unwrap();

        if let Err(e) = flush_offchain(&state, &batch, raw).await {
            warn!( error = %e, ?batch, "failed to send document to offchain, skipping..." );
        }

        if batch.count >= config.batch_size {
            if let Err(e) = flush_onchain(&state, &batch).await {
                error!( error = %e, ?batch, "error sending batch onchain, skipping..." );
            } else {
                info!(?batch, "batch sent successfully");
            }
            batch.reset();
        }
    }
    if batch.count > 0 {
        info!(?batch, "sending final batch");

        if let Err(e) = flush_onchain(&state, &batch).await {
            error!( error = %e, ?batch, "error sending final batch onchain, skipping..." );
        }
    }
}

async fn flush_offchain(state: &AppState, batch: &Batch, doc: Map<String, Value>) -> Result<()> {
    let item = Storable { id: batch.id.clone(), ord: batch.count, doc };
    state.tx.storage.send(item).await?;
    state.prometheus.elastic_queue.inc();
    Ok(())
}

async fn flush_onchain(state: &AppState, batch: &Batch) -> Result<()> {
    let (id, hash) = (batch.id.clone(), batch.hash);
    let item = Fingerprint { hash, id };
    state.tx.ethereum.send(item).await?;
    state.prometheus.ethereum_queue.inc();
    Ok(())
}
