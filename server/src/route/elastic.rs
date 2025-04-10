use crate::state::AppState;
use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/elastic/{index}", get(handle_hash))
        .with_state(state)
}

async fn handle_hash(Path(index): Path<String>, State(state): State<Arc<AppState>>) -> String {
    match state.elastic.hash(index.as_str()).await {
        Ok(result) => result,
        Err(err) => return format!("error: {}", err),
    }
}
