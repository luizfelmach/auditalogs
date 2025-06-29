use crate::{handlers::ui::serve_ui, routes, state::AppState};
use axum::Router;
use std::{process, sync::Arc};
use tokio::net;
use tower_http::cors::{Any, CorsLayer};
use tower_http::normalize_path::NormalizePathLayer;
use tracing::{error, info};

pub async fn server(state: Arc<AppState>) {
    let url = format!("{}:{}", state.config.host, state.config.port);
    let bind = net::TcpListener::bind(&url).await;

    let Ok(listener) = bind else {
        error!("failed to bind to {}: {:?}", url, bind);
        process::exit(1)
    };

    let app = create_app((*state).clone());

    match axum::serve(listener, app).await {
        Ok(_) => info!("server terminated gracefully"),
        Err(err) => error!("server encountered an error during execution: {:?}", err),
    }
}

fn create_app(state: AppState) -> Router {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    Router::new()
        .nest("/api", routes::api())
        .fallback(serve_ui)
        .layer(cors)
        .layer(NormalizePathLayer::trim_trailing_slash())
        .with_state(state)
}
