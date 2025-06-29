use std::sync::Arc;

use crate::{
    batch::Batch,
    entity::Storable,
    error::{AppError, Result},
    state::AppState,
    storage::{search::QueryExpr, Storage},
};
use anyhow::Context;
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use moka::future::Cache;
use serde::Serialize;

pub type CacheBatchResponse = Arc<Cache<String, BatchResponse>>;

#[derive(Serialize, Clone)]
pub struct BatchResponse {
    id: String,
    count: usize,
    hash: String,
}

pub async fn get_hash_storage(
    State(state): State<AppState>,
    Path(batch_id): Path<String>,
    Extension(cache): Extension<CacheBatchResponse>,
) -> Result<Json<BatchResponse>> {
    if let Some(cached) = cache.get(&batch_id).await {
        return Ok(Json(cached.clone()));
    }

    let results = state
        .storage
        .retrieve(batch_id.as_str())
        .await
        .context("An error occurrued when retrieving data from storage")?;

    if results.is_empty() {
        return Err(AppError::NotFound(
            "No records found for the given batch_id".into(),
        ));
    }

    let mut batch = Batch::new();

    results
        .iter()
        .try_for_each(|item| batch.add(&item.doc))
        .context("Error while computing the batch hash from the document contents")?;

    let response = BatchResponse {
        id: batch_id.clone(),
        count: batch.count,
        hash: batch.hash.to_hex(),
    };

    cache.insert(batch_id, response.clone()).await;

    Ok(Json(response))
}

pub async fn search_docs(
    State(state): State<AppState>,
    Json(query): Json<QueryExpr>,
) -> Result<Json<Vec<Storable>>> {
    let results = state
        .storage
        .search(query)
        .await
        .context("An error ocurrued when processing query")?;
    Ok(Json(results))
}
