mod channel;
mod client;
mod config;
mod route;
mod task;
mod utils;

use config::AppConfig;
use tokio::{net, runtime::Builder};

fn main() {
    dotenvy::dotenv().ok();

    let config = AppConfig::load().expect("Erro carregando configuração");
    println!("{:#?}", config);

    let runtime = Builder::new_multi_thread()
        .worker_threads(config.threads)
        .enable_all()
        .build()
        .expect("Failed to build runtime");

    runtime.block_on(server(config));
}

async fn server(config: AppConfig) {
    let (tx, rx) = channel::new(config.queue_size);

    tokio::spawn(task::worker(config.clone(), tx.clone(), rx.clone()));
    tokio::spawn(task::ethereum(config.clone(), tx.clone(), rx.clone()));
    tokio::spawn(task::elastic(config.clone(), tx.clone(), rx.clone()));

    let app = route::create_router(tx.clone());

    let listener = net::TcpListener::bind((config.host.clone(), config.port.clone()))
        .await
        .unwrap();

    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("Server encountered an error during execution: {err}");
    }
}
