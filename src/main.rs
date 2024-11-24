use blockchain::BlockchainStore;
use fingerprint::Fingerprint;
use queue::Queue;
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
    let queue_client = queue::rust_client::RustClient::default();
    let mut blockchain_client = blockchain::blockchain_file_client::BlockchainFileClient::default();

    queue_client
        .on_message(|data| {
            let _fp = data.fingerprint();
            Ok(())
        })
        .await?;

    Ok(())
}
