mod channel;
mod client;
mod config;
mod prometheus;
mod route;
mod setup;
mod state;
mod task;
mod utils;

use state::AppState;
use std::sync::Arc;
use tracing::{debug, info, warn};

fn main() {
    setup::log();

    info!("starting application");

    let state = setup::state();
    let runtime = setup::runtime(state.config.threads);

    runtime.block_on(app(state));
}

async fn app(state: Arc<AppState>) {
    debug!("starting tasks: worker, ethereum, elastic, and server");

    tokio::select! {
        () = task::worker(Arc::clone(&state)) => warn!("worker task exited unexpectedly"),
        () = task::ethereum(Arc::clone(&state)) => warn!("ethereum task exited unexpectedly"),
        () = task::elastic(Arc::clone(&state)) => warn!("elastic task exited unexpectedly"),
        () = task::server(Arc::clone(&state)) => warn!("server task exited unexpectedly"),
    }

    warn!("shutting down application")
}
