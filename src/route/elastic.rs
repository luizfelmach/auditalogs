use crate::state::AppState;
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
        .route("/elastic/{index}", get(handle_hash))
        .route("/elastic/search", post(handle_search))
        .with_state(state)
}

#[instrument(skip(state))]
pub async fn handle_hash(
    Path(index): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let exists = state.elastic.exists(index.as_str()).await.map_err(|e| {
        error!(
            error = %e,
            "failed to check index existence in elasticsearch"
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

    let hash = state.elastic.hash(index.as_str()).await.map_err(|e| {
        error!(
            error = %e,
            "failed to calculate index hash"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Failed to calculate hash",
                "details": e.to_string()
            })),
        )
    })?;

    Ok(Json(json!({
        "status": "success",
        "index": index,
        "hash": format!("0x{hash}"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

#[instrument(skip(state))]
pub async fn handle_search(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let search_results = state.elastic.search_query(payload).await.map_err(|e| {
        error!(
            error = %e,
            "failed to search documents by IP and date range"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Failed to search documents",
                "details": e.to_string()
            })),
        )
    })?;

    Ok(Json(json!({
        "status": "success",
        "count": search_results.len(),
        "results": search_results,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
