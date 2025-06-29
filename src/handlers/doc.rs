use crate::{entity::Document, error::Result, state::AppState};
use anyhow::Context;
use axum::{extract::State, Json};

pub async fn handle_doc(
    State(state): State<AppState>,
    Json(document): Json<Document>,
) -> Result<()> {
    state
        .tx
        .worker
        .send(document)
        .await
        .context("Failed to enqueue message")?;
    Ok(())
}
