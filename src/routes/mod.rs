pub mod blockchain;
pub mod doc;
pub mod metrics;
pub mod storage;

use crate::state::AppState;
use axum::{routing::get, Router};

pub fn api() -> Router<AppState> {
    Router::new()
        .merge(doc::routes())
        .nest("/blockchain", blockchain::routes())
        .nest("/storage", storage::routes())
        .nest("/metrics", metrics::routes())
        .route("/ping", get(ping))
}

async fn ping() -> &'static str {
    "pong"
}
