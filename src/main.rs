use blockchain::{BlockchainStore, FsBlockchain};
use fingerprint::Fingerprint;
use futures::StreamExt;
use queue::RustQueue;
use storage::{FsStorage, StorageStore};
use tracing::{error, warn, Level};
use tracing_subscriber;

mod blockchain;
mod core;
mod fingerprint;
mod queue;
mod storage;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    loop {
        match app().await {
            Ok(_) => break,
            Err(err) => error!("Something went wrong: {err}"),
        }

        warn!("Retrying in 2 seconds...");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}

async fn app() -> Result<(), Box<dyn std::error::Error>> {
    let mut queue = RustQueue::default();
    let mut blockchain = FsBlockchain::default();
    let mut storage = FsStorage::default();

    while let Some(data) = queue.next().await {
        let id = String::from("SOME_ID");
        let fingerprint = data.fingerprint();
        blockchain.store(&id, &fingerprint).await?;
        storage.store(&id, &data).await?;
    }

    Ok(())
}
