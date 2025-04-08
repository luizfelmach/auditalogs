use crate::{
    channel::{RxChannel, TxChannel},
    client::elastic::ElasticClient,
    config::AppConfig,
};

pub async fn elastic(config: AppConfig, _: TxChannel, rx: RxChannel) {
    let client = ElasticClient::new(
        "http://localhost:9200".into(),
        "elastic".into(),
        "changeme".into(),
    )
    .unwrap();
    while let Some(msg) = rx.elastic.lock().await.recv().await {
        let value = serde_json::from_str(&msg.content).unwrap();
        if let Err(e) = client.store(&msg.index, &value).await {
            eprintln!("[elastic][error]: {}", e)
        }
    }
}
