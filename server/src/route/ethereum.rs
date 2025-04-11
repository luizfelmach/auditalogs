use crate::{channel::EthereumChannelItem, state::AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{error, instrument};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ethereum", post(handle_ethereum))
        .route("/ethereum/{index}", get(handle_hash))
        .with_state(state)
}

#[instrument(skip(state))]
async fn handle_hash(
    Path(index): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let exists = state.ethereum.exists(index.clone()).await.map_err(|e| {
        error!(
            error = %e,
            "failed to check index existence in ethereum"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "failed to check index existence",
                "details": e.to_string()
            })),
        )
    })?;

    if !exists {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "not_found",
                "message": "Index not found",
                "index": index
            })),
        ));
    }

    let hash = state.ethereum.hash(index.clone()).await.map_err(|e| {
        error!(
            error = %e,
            "failed to retrieve index hash from contract"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Failed to retrieve hash from contract",
                "details": e.to_string()
            })),
        )
    })?;

    Ok(Json(json!({
        "status": "success",
        "index": index,
        "hash": hash,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

#[instrument(skip(state))]
async fn handle_ethereum(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EthereumChannelItem>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let received = payload.clone();

    match state.tx.ethereum.send(payload).await {
        Ok(_) => Ok(Json(json!({
            "status": "processing",
            "message": "Data received and being processed",
            "received": received,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }))),
        Err(err) => {
            error!(
                error = %err,
                "failed to enqueue message"
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to enqueue message",
                    "error": err.to_string(),
                    "received": received,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                })),
            ))
        }
    }
}
