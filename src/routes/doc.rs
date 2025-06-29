use crate::{handlers::doc::handle_doc, state::AppState};
use axum::{routing::post, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(handle_doc))
}
