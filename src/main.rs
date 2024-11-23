use alloy::primitives::address;
use elastic::Elastic;
use rabbitmq::Rabbitmq;
use std::error::Error;
use tracing::{error, warn, Level};
use tracing_subscriber;

mod elastic;
mod ethereum;
mod hashing;
mod rabbitmq;

const ELASTIC_URL: &str = "http://localhost:9200";
const ELASTIC_USER: &str = "elastic";
const ELASTIC_PASSWORD: &str = "changeme";
const RABBIT_URL: &str = "amqp://rabbit:changeme@localhost:5672";
const RABBIT_QUEUE: &str = "queue";
const RABBIT_BATCH: usize = 100;
const RPC_URL: &str = "http://localhost:8545";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    loop {
        match listen().await {
            Ok(_) => break,
            Err(err) => {
                error!("Something went wrong: {err}");
            }
        }
        let seconds = 2;
        warn!("Retrying in {seconds} seconds...");
        tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
    }
}

async fn listen() -> Result<(), Box<dyn Error>> {
    let elastic = Elastic::new(ELASTIC_URL, ELASTIC_USER, ELASTIC_PASSWORD)?;
    let mut rabbit = Rabbitmq::new(RABBIT_URL, RABBIT_QUEUE, RABBIT_BATCH).await?;
    let _contract = ethereum::Contract(
        RPC_URL,
        address!("5fbdb2315678afecb367f032d93f642f64180aa3"),
    )?;

    rabbit
        .consumer(|messages: Vec<Vec<u8>>| {
            let hash = hashing::fingerprint(&messages);

            let result = elastic.store_data(&hash, &messages);

            if let Err(error) = result {
                return Err(error);
            }

            Ok(())
        })
        .await;

    Ok(())
}
