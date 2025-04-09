mod channel;
mod client;
mod config;
mod route;
mod state;
mod task;
mod utils;

use state::AppState;
use std::{process, sync::Arc};
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
    let url = format!("{}:{}", state.config.host, state.config.port);
    let bind = net::TcpListener::bind(&url).await;

    let Ok(listener) = bind else {
        log::error!("Failed to bind to {}: {:?}", url, bind);
        process::exit(1);
    };

    log::info!("Server listening on {}", url);

    match axum::serve(listener, app).await {
        Ok(_) => log::info!("Server terminated gracefully"),
        Err(err) => log::error!("Server encountered an error during execution: {:?}", err),
    }
}
