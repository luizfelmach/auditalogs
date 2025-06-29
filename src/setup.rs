use crate::{
    channel, client::ethereum::EthereumClient, config::AppConfig, prometheus::Prometheus, state::AppState,
    storage::elasticsearch::ElasticsearchAdapter,
};
use clap::Parser;
use std::{env, process, sync::Arc};
use tokio::runtime::{Builder, Runtime};
use tracing::{debug, error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
}

pub fn log() {
    let level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", format!("{},alloy=error,hyper=error,reqwest=error,axum=error", level));
    tracing_subscriber::fmt::init();

    debug!(LOG_LEVEL = level);
}

pub fn runtime(threads: usize) -> Runtime {
    debug!("creating tokio runtime with {} worker threads", threads);

    let runtime = Builder::new_multi_thread().worker_threads(threads).enable_all().build();

    match runtime {
        Ok(runtime) => runtime,
        Err(err) => {
            error!("failed to build runtime: {err}");
            process::exit(1);
        }
    }
}

pub fn state() -> Arc<AppState> {
    let args = Args::parse();
    let config = AppConfig::load(args.config);
    let Ok(config) = config else {
        error!("error reading config file: {:?}", config);
        process::exit(1);
    };

    let (tx, rx) = channel::new(config.queue_size);

    let storage = ElasticsearchAdapter::new(config.elastic.url.clone(), config.elastic.username.clone(), config.elastic.password.clone());

    let Ok(storage) = storage else {
        error!("error creating elastic client: {:?}", storage);
        process::exit(1);
    };

    let ethereum = EthereumClient::new(config.ethereum.url.clone(), config.ethereum.contract.clone(), config.ethereum.private_key.clone());

    let Ok(ethereum) = ethereum else {
        error!("error creating ethereum client: {:?}", ethereum);
        process::exit(1);
    };

    let prometheus = Prometheus::new();

    return Arc::new(AppState { config, tx, rx, storage, ethereum, prometheus });
}
