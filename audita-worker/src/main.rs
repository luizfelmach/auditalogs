use std::{collections::HashMap, string, sync::Arc, thread};

use alloy::{
    hex,
    primitives::{Address, B256},
    providers::ProviderBuilder,
    sol,
};
use chrono::Utc;
use elasticsearch::{
    auth::Credentials,
    http::{
        request::JsonBody,
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    BulkOperation, BulkParts, Elasticsearch,
};
use futures::lock::Mutex;
use rand::{seq::index, Rng};
use redis::Commands;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const WORKER_NAME: &str = "firewall";

const REDIS_URL: &str = "redis://127.0.0.1";
const REDIS_KEY: &str = "logs";
const BATCH_SIZE: usize = 1000;

const ELASTIC_URL: &str = "http://localhost:9200";
const ELASTIC_USERNAME: &str = "elastic";
const ELASTIC_PASSWORD: &str = "changeme";

const RPC_URL: &str = "http://localhost:8545";
const RPC_CONTRACT: &str = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
const WORKERS: usize = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = vec![];

    for _ in 0..WORKERS {
        let handle = tokio::spawn(async move {
            let redis = redis::Client::open(REDIS_URL).unwrap();
            let mut redis = redis.get_connection().unwrap();
            let mut messages: Vec<String> = vec![];
            let mut accumulated_hash = String::new();
            loop {
                let msg: Option<(String, String)> = redis.brpop(REDIS_KEY, 0.0).unwrap();

                if let Some((_, value)) = msg {
                    let mut hasher = Sha256::new();
                    hasher.update(accumulated_hash.as_bytes());
                    hasher.update(value.as_bytes());
                    accumulated_hash = format!("{:x}", hasher.finalize());
                    messages.push(value);
                }
                if messages.len() >= BATCH_SIZE {
                    process_messages(messages.clone(), accumulated_hash.clone())
                        .await
                        .unwrap();
                    messages.clear();
                    accumulated_hash.clear();
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}

async fn process_messages(
    messages: Vec<String>,
    hash: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let index_name = generate_index_name();
    let url = Url::parse(ELASTIC_URL)?;
    let pool = SingleNodeConnectionPool::new(url);
    let credentials = Credentials::Basic(ELASTIC_USERNAME.into(), ELASTIC_PASSWORD.into());
    let transport = TransportBuilder::new(pool).auth(credentials).build()?;
    let elastic = Elasticsearch::new(transport);

    send_to_elasticsearch(messages, &index_name, &elastic)
        .await
        .unwrap();

    println!("{} -- {}", index_name, hash);

    let provider = ProviderBuilder::new().on_http(RPC_URL.parse()?);

    let contract_address: Address = RPC_CONTRACT.parse()?;

    let contract = Auditability::new(contract_address, provider);

    let hash = B256::from_slice(&hex::decode(&hash)?);
    let tx_hash = contract.store(index_name, hash).send().await?;
    let receipt = tx_hash.get_receipt().await?; // Aguarda o recibo da transação
                                                //println!("Recibo: {:?}", receipt);
    Ok(())
}

fn generate_index_name() -> String {
    let current_time = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    format!("{}-{}-{}", WORKER_NAME, current_time, random_string).to_lowercase()
}

async fn send_to_elasticsearch(
    messages: Vec<String>,
    index_name: &str,
    elastic: &Elasticsearch,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ops: Vec<BulkOperation<Value>> = Vec::new();

    for (i, d) in messages.iter().enumerate() {
        let json = serde_json::from_str(&d)?;
        ops.push(BulkOperation::create(json).id(i.to_string()).into());
    }

    let response = elastic
        .bulk(BulkParts::Index(&index_name))
        .body(ops)
        .send()
        .await?;

    if !response.status_code().is_success() {
        let error_body: Value = response.json().await?;
        eprintln!("Bulk request failed: {:?}", error_body);
    }

    Ok(())
}

sol! {
    #[sol(rpc)]
    contract Auditability {
        function store(string index, bytes32 root) external;
        function proof(string index, bytes32 root) external view returns (bool);
    }
}
