use crate::state::AppState;
use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::error;

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

    if let Err(err) = state.tx.worker.send(payload.to_string()).await {
        error!("failed to enqueue message to worker: {}", err);
        return Json(json!({
            "message": "Failed to enqueue message",
            "received": received,
        }));
    }

    return Json(json!({
        "message": "Data received and being processed.",
        "received": received
    }));
}
