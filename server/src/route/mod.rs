pub mod elastic;
pub mod ethereum;
pub mod logs;

use crate::state::AppState;
use axum::Router;
use std::sync::Arc;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(logs::create_router(Arc::clone(&state)))
        .merge(ethereum::create_router(Arc::clone(&state)))
        .merge(elastic::create_router(Arc::clone(&state)))
}
