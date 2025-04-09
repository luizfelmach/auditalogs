use std::{env, process, sync::Arc};

use tokio::runtime::{Builder, Runtime};
use tracing::{debug, error};

use crate::{
    channel,
    client::{elastic::ElasticClient, ethereum::EthereumClient},
    config::AppConfig,
    state::AppState,
};

pub fn log() {
    let level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    env::set_var(
        "RUST_LOG",
        format!("{},alloy=error,hyper=error,reqwest=error,axum=error", level),
    );
    tracing_subscriber::fmt::init();

    debug!(LOG_LEVEL = level);
}

pub fn runtime(threads: usize) -> Runtime {
    debug!("creating tokio runtime with {} worker threads", threads);

    let runtime = Builder::new_multi_thread()
        .worker_threads(threads)
        .enable_all()
        .build();

    match runtime {
        Ok(runtime) => runtime,
        Err(err) => {
            error!("failed to build runtime: {err}");
            process::exit(1);
        }
    }
}

pub fn state() -> Arc<AppState> {
    let config = AppConfig::load("Config.toml");

    let (tx, rx) = channel::new(config.queue_size);

    let result = ElasticClient::new(
        config.elastic.url.clone(),
        config.elastic.username.clone(),
        config.elastic.password.clone(),
    );

    let Ok(elastic) = result else {
        error!("Could not create elastic client");
        process::exit(1);
    };

    let ethereum = EthereumClient::new(
        config.ethereum.url.clone(),
        config.ethereum.contract.clone(),
        config.ethereum.private_key.clone(),
    );

    let Ok(ethereum) = ethereum else {
        error!("Could not create ethereum client");
        process::exit(1);
    };

    return Arc::new(AppState {
        config,
        tx,
        rx,
        elastic,
        ethereum,
    });
}
