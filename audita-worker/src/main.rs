mod cli;
mod config;
mod elastic_client;
mod eth_client;
mod tx_manager;
mod utils;

use axum::{extract::State, routing::post, Json, Router};
use elastic_client::ElasticClient;
use eth_client::EthClient;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};
use utils::{fingerprint, generate_index};

const BATCH_SIZE: usize = 5;
const BATCH_ETHEREUM: usize = 100;

#[tokio::main]
async fn main() {
    let (worker_tx, worker_rx) = mpsc::channel(100);
    let (elastic_tx, elastic_rx) = mpsc::channel(100);
    let (ethereum_tx, ethereum_rx) = mpsc::channel(100);

    tokio::spawn(task_worker(worker_rx, elastic_tx, ethereum_tx.clone()));
    tokio::spawn(task_elastic(elastic_rx));
    tokio::spawn(task_ethereum(ethereum_rx));

    let app_state = Arc::new(AppState {
        worker_tx: Arc::new(worker_tx),
        ethereum_tx: Arc::new(ethereum_tx),
    });

    let app = Router::new()
        .route("/", post(handle_logs))
        .route("/eth", post(handle_eth))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct AppState {
    worker_tx: Arc<Sender<WorkerChannelItem>>,
    ethereum_tx: Arc<Sender<EthereumChannelItem>>,
}

type WorkerChannelItem = Value;

#[derive(Clone, Serialize, Deserialize)]
struct EthereumChannelItem {
    index: String,
    hash: String,
}

struct ElasticChannelItem {
    index: String,
    content: Value,
}

async fn task_worker(
    mut worker_rx: Receiver<WorkerChannelItem>,
    elastic_tx: Sender<ElasticChannelItem>,
    ethereum_tx: Sender<EthereumChannelItem>,
) {
    let mut counter = 0;
    let mut hash = String::new();
    let mut index = generate_index(&"worker".into());

    while let Some(msg) = worker_rx.recv().await {
        hash = fingerprint(&hash, &msg.to_string());
        counter += 1;

        let item = ElasticChannelItem {
            index: index.clone(),
            content: msg.clone(),
        };
        match elastic_tx.send(item).await {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to enqueue message to elastic: {}", e),
        }

        if counter >= BATCH_SIZE {
            let item = EthereumChannelItem {
                index: index.clone(),
                hash: hash.clone(),
            };
            match ethereum_tx.send(item).await {
                Ok(_) => (),
                Err(e) => eprintln!("Failed to enqueue message to ethereum: {}", e),
            }

            counter = 0;
            hash = String::new();
            index = generate_index(&"worker".into());
        }
    }
}

async fn task_elastic(mut elastic_rx: Receiver<ElasticChannelItem>) {
    let elastic_client = ElasticClient::new(
        "http://localhost:9200".into(),
        "elastic".into(),
        "changeme".into(),
    )
    .unwrap();

    while let Some(msg) = elastic_rx.recv().await {
        println!("ELASTIC: {}  {}", msg.index, msg.content);
        let result = elastic_client.store_once(&msg.index, &msg.content).await;
        if let Err(e) = result {
            eprintln!("[Elastic] [Error]: {}", e)
        }
    }
}

async fn task_ethereum(mut ethereum_rx: Receiver<EthereumChannelItem>) {
    let eth_client = EthClient::new(
        "http://localhost:8545".into(),
        "0x42699A7612A82f1d9C36148af9C77354759b210b".into(),
        "f4bbe9c1f7371ab4654130f28a77e36c40ba618fc4ece325fac70b7f5965f8bc".into(),
    )
    .await
    .unwrap();

    let eth_client = Arc::new(eth_client);

    let nonce_queue = Arc::new(Mutex::new(VecDeque::new()));
    let mut nonce = eth_client.nonce().await.unwrap();
    let mut buffer = Vec::new();

    while let Some(msg) = ethereum_rx.recv().await {
        buffer.push(msg);
        nonce_queue.lock().await.push_back(nonce);
        nonce += 1;

        if buffer.len() >= BATCH_ETHEREUM {
            let mut handles = Vec::new();
            for tx in buffer.iter() {
                let nonce_queue_clone = Arc::clone(&nonce_queue);
                let nonce_tx = nonce_queue.lock().await.pop_front().unwrap();
                let eth_client_clone = Arc::clone(&eth_client);
                let tx_clone = tx.clone();

                let handle = tokio::spawn(async move {
                    let result = eth_client_clone
                        .store(nonce_tx.clone(), &tx_clone.index, &tx_clone.hash)
                        .await;
                    match result {
                        Ok(_) => println!("OK {}", nonce_tx),
                        Err(e) => {
                            println!("Aconteceu um erro no nonce {}: {}", nonce_tx, e);
                            nonce_queue_clone.lock().await.push_front(nonce_tx);
                        }
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                match handle.await {
                    Ok(_) => (),
                    Err(_) => (),
                }
            }

            buffer.clear();
        }
    }
}

async fn handle_logs(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let received = payload.clone();
    match state.worker_tx.send(payload).await {
        Ok(_) => Json(json!({
            "message": "Data received and being processed.",
            "received": received
        })),
        Err(_) => Json(json!({
            "message": "Failed to enqueue message",
            "received": received
        })),
    }
}

async fn handle_eth(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EthereumChannelItem>,
) -> Json<Value> {
    let received = payload.clone();
    match state.ethereum_tx.send(payload).await {
        Ok(_) => Json(json!({
            "message": "Data received and being processed.",
            "received": received
        })),
        Err(_) => Json(json!({
            "message": "Failed to enqueue message",
            "received": received
        })),
    }
}
