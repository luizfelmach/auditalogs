mod channel;
mod client;
mod config;
mod route;
mod task;
mod utils;

use config::AppConfig;
use tokio::{net, runtime::Builder};

fn main() {
    env_logger::init();

    log::info!("Starting application");

    let config = AppConfig::load("Config.toml");

    log::info!("Configuration loaded from Config.toml");

    let runtime = Builder::new_multi_thread()
        .worker_threads(config.threads)
        .enable_all()
        .build()
        .expect("Failed to build runtime");

    log::info!("Runtime built with {} worker threads", config.threads);

    runtime.block_on(server(config));
}

async fn server(config: AppConfig) {
    log::info!("Server starting");

    let (tx, rx) = channel::new(config.queue_size);

    tokio::spawn(task::worker(config.clone(), tx.clone(), rx.clone()));
    log::info!("Spawned worker task");

    tokio::spawn(task::ethereum(config.clone(), tx.clone(), rx.clone()));
    log::info!("Spawned ethereum task");

    tokio::spawn(task::elastic(config.clone(), tx.clone(), rx.clone()));
    log::info!("Spawned elastic task");

    let app = route::create_router(tx.clone());

    let listener = match net::TcpListener::bind((config.host.clone(), config.port.clone())).await {
        Ok(listener) => listener,
        Err(err) => {
            let endpoint = format!("{}:{}", config.host, config.port);
            log::error!("Failed to bind to {}: {:?}", endpoint, err);
            return;
        }
    };

    log::info!("Server listening on {}:{}", config.host, config.port);

    match axum::serve(listener, app).await {
        Ok(_) => log::info!("Server terminated gracefully"),
        Err(err) => log::error!("Server encountered an error during execution: {:?}", err),
    }
}
