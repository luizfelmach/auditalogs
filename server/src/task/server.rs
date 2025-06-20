use crate::{route, state::AppState};
use axum::{
    http::{HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Response},
    Router,
};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::{borrow::Cow, process, sync::Arc};
use tokio::net;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

#[derive(RustEmbed)]
#[folder = "./static"]
struct Asset;

async fn static_handler(uri: Uri) -> Response {
    let path = uri.path();
    let path = if path == "/" || path.is_empty() {
        "index.html"
    } else {
        &path[1..]
    };

    match Asset::get(path) {
        Some(content) => {
            let body = match content.data {
                Cow::Borrowed(b) => b.to_vec(),
                Cow::Owned(o) => o,
            };
            let mime = from_path(path).first_or_octet_stream();
            let mut headers = HeaderMap::new();
            headers.insert("content-type", mime.to_string().parse().unwrap());
            (headers, body).into_response()
        }
        None => {
            if path != "index.html" {
                if let Some(index) = Asset::get("index.html") {
                    let body = match index.data {
                        Cow::Borrowed(b) => b.to_vec(),
                        Cow::Owned(o) => o,
                    };
                    let mut headers = HeaderMap::new();
                    headers.insert("content-type", "text/html".parse().unwrap());
                    return (headers, body).into_response();
                }
            }
            (StatusCode::NOT_FOUND, "404 Not Found").into_response()
        }
    }
}

pub async fn server(state: Arc<AppState>) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", route::create_router(Arc::clone(&state)))
        .fallback(static_handler)
        .layer(cors);

    let url = format!("{}:{}", state.config.host, state.config.port);
    let bind = net::TcpListener::bind(&url).await;
    let Ok(listener) = bind else {
        error!("failed to bind to {}: {:?}", url, bind);
        process::exit(1)
    };
    info!("server listening on {}", url);
    match axum::serve(listener, app).await {
        Ok(_) => info!("server terminated gracefully"),
        Err(err) => error!("server encountered an error during execution: {:?}", err),
    }
}
