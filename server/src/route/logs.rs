use crate::state::AppState;
use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(handle_logs))
        .with_state(state)
}

async fn handle_logs(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let received = payload.clone();
    match state.tx.worker.send(payload.to_string()).await {
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
