use std::{sync::Arc, time::Duration};

use crate::{
    handlers::storage::{get_hash_storage, search_docs, CacheBatchResponse},
    state::AppState,
};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use moka::future::Cache;

pub fn routes() -> Router<AppState> {
    let cache = Cache::builder()
        .time_to_live(Duration::from_secs(60))
        .max_capacity(1000)
        .build();
    let cache: CacheBatchResponse = Arc::new(cache);

    Router::new()
        .route("/search", post(search_docs))
        .route("/hash/{batch_id}", get(get_hash_storage))
        .layer(Extension(cache))
}
