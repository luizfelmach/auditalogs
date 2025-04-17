use crate::state::AppState;
use axum::{extract::State, routing::get, Router};
use std::sync::Arc;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/metrics", get(handle_metrics))
        .with_state(state)
}

async fn handle_metrics(State(state): State<Arc<AppState>>) -> String {
    let gather = state.prometheus.gather();
    return gather;
}
