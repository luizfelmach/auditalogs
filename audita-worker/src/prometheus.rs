use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use lazy_static::lazy_static;
use prometheus::{register_counter, register_gauge, register_histogram, Counter, Gauge, Histogram};

lazy_static! {
    pub static ref ELASTIC_SUCCESS: Counter = register_counter!(
        "elastic_store_success_total",
        "Total de armazenamentos bem-sucedidos no Elasticsearch"
    )
    .unwrap();
    pub static ref ELASTIC_ERRORS: Counter = register_counter!(
        "elastic_store_errors_total",
        "Total de erros no Elasticsearch"
    )
    .unwrap();
    pub static ref ETHEREUM_SUCCESS: Counter = register_counter!(
        "ethereum_store_success_total",
        "Total de armazenamentos bem-sucedidos na blockchain"
    )
    .unwrap();
    pub static ref ETHEREUM_ERRORS: Counter = register_counter!(
        "ethereum_store_errors_total",
        "Total de erros na blockchain"
    )
    .unwrap();
    pub static ref WORKER_QUEUE: Gauge = register_gauge!(
        "worker_queue_size",
        "Tamanho atual da fila de processamento"
    )
    .unwrap();
    pub static ref ETHEREUM_QUEUE: Gauge = register_gauge!(
        "ethereum_queue_size",
        "Tamanho atual da fila de processamento"
    )
    .unwrap();
    pub static ref ELASTIC_QUEUE: Gauge = register_gauge!(
        "elastic_queue_size",
        "Tamanho atual da fila de processamento"
    )
    .unwrap();
    pub static ref PROCESSING_TIME: Histogram = register_histogram!(
        "request_processing_seconds",
        "Tempo de processamento das requisições",
        vec![0.1, 0.5, 1.0, 2.0]
    )
    .unwrap();
}

pub fn prometheus_metrics() -> PrometheusMetrics {
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    prometheus
        .registry
        .register(Box::new(ELASTIC_SUCCESS.clone()))
        .unwrap();
    prometheus
        .registry
        .register(Box::new(ELASTIC_ERRORS.clone()))
        .unwrap();
    prometheus
        .registry
        .register(Box::new(ETHEREUM_SUCCESS.clone()))
        .unwrap();
    prometheus
        .registry
        .register(Box::new(ETHEREUM_ERRORS.clone()))
        .unwrap();
    prometheus
        .registry
        .register(Box::new(WORKER_QUEUE.clone()))
        .unwrap();
    prometheus
        .registry
        .register(Box::new(ELASTIC_QUEUE.clone()))
        .unwrap();
    prometheus
        .registry
        .register(Box::new(ETHEREUM_QUEUE.clone()))
        .unwrap();

    prometheus
        .registry
        .register(Box::new(PROCESSING_TIME.clone()))
        .unwrap();

    return prometheus;
}
