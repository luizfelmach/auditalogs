mod channel;
mod client;
mod config;
mod route;
mod state;
mod task;
mod utils;

use state::AppState;
use std::{env, process, sync::Arc};
use tokio::{net, runtime::Builder};
use tracing::{debug, error, info};

fn main() {
    logging();

    info!("Starting application");

    let state = AppState::new();
    let state = Arc::new(state);

    debug!(
        "Creating Tokio runtime with {} worker threads",
        state.config.threads
    );

    let runtime = Builder::new_multi_thread()
        .worker_threads(state.config.threads)
        .enable_all()
        .build();

    let Ok(runtime) = runtime else {
        error!("Failed to build runtime. Exiting");
        process::exit(1);
    };

    runtime.block_on(server(state));
}

async fn server(state: Arc<AppState>) {
    info!("Starting server");

    tokio::spawn(task::worker(Arc::clone(&state)));
    info!("Spawned worker task");

    tokio::spawn(task::ethereum(Arc::clone(&state)));
    info!("Spawned ethereum task");

    tokio::spawn(task::elastic(Arc::clone(&state)));
    info!("Spawned elastic task");

    let app = route::create_router(Arc::clone(&state));
    let url = format!("{}:{}", state.config.host, state.config.port);
    let bind = net::TcpListener::bind(&url).await;

    let Ok(listener) = bind else {
        error!("Failed to bind to {}: {:?}", url, bind);
        process::exit(1);
    };

    info!("Server listening on {}", url);

    match axum::serve(listener, app).await {
        Ok(_) => info!("Server terminated gracefully"),
        Err(err) => error!("Server encountered an error during execution: {:?}", err),
    }
}

fn logging() {
    let level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    env::set_var(
        "RUST_LOG",
        format!("{},alloy=error,hyper=error,reqwest=error,axum=error", level),
    );
    tracing_subscriber::fmt::init();
}
