mod channel;
mod client;
mod config;
mod route;
mod state;
mod task;
mod utils;

use state::AppState;
use std::sync::Arc;
use tokio::{net, runtime::Builder};

fn main() {
    env_logger::init();

    log::info!("Starting application");

    let state = AppState::new();
    let state = Arc::new(state);

    log::info!("Configuration loaded from Config.toml");

    let runtime = Builder::new_multi_thread()
        .worker_threads(state.config.threads)
        .enable_all()
        .build()
        .expect("Failed to build runtime");

    log::info!("Runtime built with {} worker threads", state.config.threads);

    runtime.block_on(server(state));
}

async fn server(state: Arc<AppState>) {
    log::info!("Server starting");

    tokio::spawn(task::worker(Arc::clone(&state)));
    log::info!("Spawned worker task");

    tokio::spawn(task::ethereum(Arc::clone(&state)));
    log::info!("Spawned ethereum task");

    tokio::spawn(task::elastic(Arc::clone(&state)));
    log::info!("Spawned elastic task");

    let app = route::create_router(Arc::clone(&state));

    let host = state.config.host.clone();
    let port = state.config.port.clone();

    let listener = match net::TcpListener::bind((host.clone(), port.clone())).await {
        Ok(listener) => listener,
        Err(err) => {
            let endpoint = format!("{}:{}", host, port);
            log::error!("Failed to bind to {}: {:?}", endpoint, err);
            return;
        }
    };

    log::info!("Server listening on {}:{}", host, port);

    match axum::serve(listener, app).await {
        Ok(_) => log::info!("Server terminated gracefully"),
        Err(err) => log::error!("Server encountered an error during execution: {:?}", err),
    }
}
