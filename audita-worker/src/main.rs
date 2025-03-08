mod cli;
mod config;
mod elastic_client;
mod eth_client;
mod utils;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use elastic_client::ElasticClient;
use eth_client::EthClient;
use serde_json::Value;
use std::{sync::Arc, time::Instant};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    let config = config::parse(args.config);
    println!("{}", config);

    let (sender_worker, receiver_worker) = mpsc::channel(config.queue_worker);
    let (sender_elastic, receiver_elastic) = mpsc::channel(config.queue_elastic);
    let (sender_ethereum, receiver_ethereum) = mpsc::channel(config.queue_ethereum);

    tokio::spawn(async move {
        thread_worker(receiver_worker, sender_ethereum, sender_elastic).await;
    });
    tokio::spawn(async move {
        thread_sender_ethereum(receiver_ethereum).await;
    });
    tokio::spawn(async move {
        thread_sender_elastic(receiver_elastic).await;
    });

    let app = web::Data::new(Arc::new(AppState {
        sender: sender_worker,
    }));

    HttpServer::new(move || App::new().app_data(app.clone()).service(receive))
        .workers(config.dispatchers)
        .bind(("127.0.0.1", config.port))?
        .run()
        .await
}

struct HashQueueItem {
    index: String,
    hash: String,
}

struct BatchLogsQueueItem {
    index: String,
    content: Vec<Value>,
}

async fn thread_sender_elastic(mut receiver: Receiver<BatchLogsQueueItem>) {
    let args = cli::Args::parse();
    let config = config::parse(args.config);

    let elastic_client = ElasticClient::new(
        config.elastic.url,
        config.elastic.username,
        config.elastic.password,
    )
    .unwrap();

    while let Some(msg) = receiver.recv().await {
        if !args.disable_elastic {
            let start_time = Instant::now();
            let result = elastic_client.store(&msg.index, &msg.content).await;
            match result {
                Ok(_) => {
                    let elapsed_time = start_time.elapsed();
                    println!("[Sender Elastic]: {} {:?}", msg.index, elapsed_time)
                }
                Err(e) => eprintln!("[Sender Elastic] [Error]: {}", e),
            }
        }
    }
}

async fn thread_sender_ethereum(mut receiver: Receiver<HashQueueItem>) {
    let args = cli::Args::parse();
    let config = config::parse(args.config);

    let eth_client = EthClient::new(
        config.ethereum.url,
        config.ethereum.contract,
        config.ethereum.primary_key,
    )
    .await
    .unwrap();

    while let Some(msg) = receiver.recv().await {
        if !args.disable_ethereum {
            let start_time = Instant::now();
            let result = eth_client.store(&msg.index, &msg.hash).await;
            match result {
                Ok(_) => {
                    let elapsed_time = start_time.elapsed();
                    println!("[Sender Ethereum]: {} {:?}", msg.index, elapsed_time)
                }
                Err(e) => eprintln!("[Sender Ethereum] [Error]: {}", e),
            }
        }
    }
}

async fn thread_worker(
    mut receiver: Receiver<Value>,
    sender_blockchain: Sender<HashQueueItem>,
    sender_elastic: Sender<BatchLogsQueueItem>,
) {
    let args = cli::Args::parse();
    let config = config::parse(args.config);

    let mut buffer = Vec::new();

    while let Some(msg) = receiver.recv().await {
        buffer.push(msg);

        if buffer.len() >= config.batch {
            let index = utils::generate_index(&config.name);
            let hash = utils::fingerprint(&buffer);

            let item = HashQueueItem {
                index: index.clone(),
                hash: hash.clone(),
            };
            match sender_blockchain.send(item).await {
                Ok(_) => (),
                Err(e) => eprintln!("Failed to enqueue message to blockchain sender: {}", e),
            }

            let item = BatchLogsQueueItem {
                index: index.clone(),
                content: buffer.clone(),
            };
            match sender_elastic.send(item).await {
                Ok(_) => (),
                Err(e) => eprintln!("Failed to enqueue message to elastic sender: {}", e),
            }

            buffer.clear();
        }
    }
}

struct AppState {
    sender: mpsc::Sender<Value>,
}

#[post("/")]
async fn receive(app: web::Data<Arc<AppState>>, data: web::Json<Value>) -> impl Responder {
    if let Err(_) = app.sender.send(data.clone()).await {
        return HttpResponse::InternalServerError().body("Failed to enqueue message.");
    }
    HttpResponse::Ok().body("Data received and being processed.")
}
