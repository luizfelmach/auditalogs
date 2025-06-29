use crate::{handlers::blockchain::get_hash_blockchain, state::AppState};
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/hash/{batch_id}", get(get_hash_blockchain))
}
