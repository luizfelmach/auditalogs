mod channel;
mod client;
mod route;
mod task;
mod utils;

use tokio::net;

#[tokio::main]
async fn main() {
    let (tx, rx) = channel::new();

    tokio::spawn(task::worker(tx.clone(), rx.clone()));
    tokio::spawn(task::ethereum(tx.clone(), rx.clone()));
    tokio::spawn(task::elastic(tx.clone(), rx.clone()));

    let app = route::create_router(tx.clone());

    let listener = net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind tcp listener on 0.0.0.0:8080");

    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("Server encountered an error during execution: {err}");
    }
}
