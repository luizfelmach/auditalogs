use crate::{route, state::AppState};
use std::{process, sync::Arc};
use tokio::net;
use tracing::{error, info};

pub async fn server(state: Arc<AppState>) {
    let app = route::create_router(Arc::clone(&state));
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
