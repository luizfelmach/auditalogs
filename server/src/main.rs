mod channel;
mod client;
mod config;
mod log;
mod route;
mod state;
mod task;
mod utils;

use state::AppState;
use std::{process, sync::Arc};
use tokio::runtime::Builder;
use tracing::{debug, error, info, warn};

fn main() {
    log::setup();

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

    runtime.block_on(app(state));
}

async fn app(state: Arc<AppState>) {
    debug!("Starting tasks: worker, ethereum, elastic, and server");

    tokio::select! {
        () = task::worker(Arc::clone(&state)) => {
            warn!("Worker task exited unexpectedly. Shutting down application.");
        },
        () = task::ethereum(Arc::clone(&state)) => {
            warn!("Ethereum task exited unexpectedly. Shutting down application.");
        },
        () = task::elastic(Arc::clone(&state)) => {
            warn!("Elastic task exited unexpectedly. Shutting down application.");
        },
        () = task::server(Arc::clone(&state)) => {
            warn!("Server task exited unexpectedly. Shutting down application.");
        },
    }
}
