use crate::{handlers::metrics::get_metrics, state::AppState};
use axum::{routing::get, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(get_metrics))
}
