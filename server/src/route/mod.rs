pub mod elastic;
pub mod ethereum;
pub mod logs;

use crate::channel::TxChannel;
use axum::Router;
use std::sync::Arc;

pub fn create_router(tx: TxChannel) -> Router {
    Router::new()
        .merge(logs::create_router(Arc::new(tx.clone())))
        .merge(ethereum::create_router(Arc::new(tx.clone())))
}
