use futures::{task::Poll, Stream, StreamExt};
use queue::RustQueue;

mod blockchain;
mod core;
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
    let mut queue = RustQueue::default();

    while let Some(data) = queue.next().await {
        println!("OK")
    }

    Ok(())
}
