use crate::{channel::EthereumChannelItem, state::AppState};
use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ethereum", post(handle_ethereum))
        .with_state(state)
}

async fn handle_ethereum(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EthereumChannelItem>,
) -> Json<Value> {
    let received = payload.clone();
    match state.tx.ethereum.send(payload).await {
        Ok(_) => Json(json!({
            "message": "Data received and being processed.",
            "received": received
        })),
        Err(_) => Json(json!({
            "message": "Failed to enqueue message",
            "received": received
        })),
    }
}
