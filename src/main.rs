use blockchain::BlockchainStore;
use fingerprint::Fingerprint;
use futures::StreamExt;
use queue::{Message, RabbitmqQueue};
use storage::StorageStore;

mod blockchain;
mod fingerprint;
mod queue;
mod storage;

#[tokio::main]
async fn main() {
    if let Err(err) = app().await {
        eprintln!("Something went wrong: {err}")
    }
}

async fn app() -> Result<(), Box<dyn std::error::Error>> {
    let mut queue = RabbitmqQueue::default();
    let mut blockchain = blockchain::FsBlockchain::default();
    let mut storage = storage::FsStorage::default();

    queue.connect().await?;

    let mut batching: Vec<Message> = vec![];
    let batch_size = 100;

    while let Some(message) = queue.next().await {
        let message = match message {
            Ok(message) => message,
            Err(err) => return Err(err),
        };
        batching.push(message);

        if batching.len() < batch_size {
            continue;
        }

        let flat: Vec<Vec<u8>> = batching.iter().map(|m| m.data.clone()).collect();
        let fingerprint = flat.fingerprint();

        blockchain.store(&"SOME_ID".into(), &fingerprint).await?;
        storage.store(&"SOME_ID".into(), &flat).await?;

        for m in batching.iter() {
            m.acker.queue_ack()?;
        }

        batching.clear();
    }

    Ok(())
}
