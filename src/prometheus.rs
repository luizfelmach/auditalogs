use prometheus::{Counter, Encoder, Gauge, Registry, TextEncoder};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Prometheus {
    pub registry: Registry,
    pub logs: Arc<Counter>,
    pub logs_queue: Arc<Gauge>,
    pub elastic_queue: Arc<Gauge>,
    pub ethereum_queue: Arc<Gauge>,
}

impl Prometheus {
    pub fn new() -> Self {
        let registry = Registry::new();

        let logs = Counter::new("logs_total", "Total logs sent").unwrap();
        let logs_queue = Gauge::new("logs_queue", "Logs waiting in queue").unwrap();
        let elastic_queue = Gauge::new("elastic_queue", "ElasticSearch queue size").unwrap();
        let ethereum_queue = Gauge::new("ethereum_queue", "Ethereum queue size").unwrap();

        registry.register(Box::new(logs.clone())).unwrap();
        registry.register(Box::new(logs_queue.clone())).unwrap();
        registry.register(Box::new(elastic_queue.clone())).unwrap();
        registry.register(Box::new(ethereum_queue.clone())).unwrap();

        Self {
            registry,
            logs: Arc::new(logs),
            logs_queue: Arc::new(logs_queue),
            elastic_queue: Arc::new(elastic_queue),
            ethereum_queue: Arc::new(ethereum_queue),
        }
    }

    pub fn gather(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}
