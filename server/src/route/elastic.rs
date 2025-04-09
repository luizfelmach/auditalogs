use crate::{state::AppState, utils::fingerprint};
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
    let result = state.elastic.retrieve(index.as_str()).await;
    let Ok(items) = result else {
        return format!("Error: {}", result.unwrap_err());
    };

    let mut hash = String::new();

    for item in items {
        let Some(source) = item.get("_source") else {
            continue;
        };
        hash = fingerprint(&hash, &source.to_string());
    }

    return hash;
}
