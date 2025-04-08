use crate::{
    channel::{RxChannel, TxChannel},
    client::elastic::ElasticClient,
    config::AppConfig,
};

pub async fn elastic(config: AppConfig, _: TxChannel, rx: RxChannel) {
    let elastic = config.elastic;
    let client = ElasticClient::new(elastic.url, elastic.username, elastic.password).unwrap();

    while let Some(msg) = rx.elastic.lock().await.recv().await {
        if elastic.disable {
            return;
        }

        let value = serde_json::from_str(&msg.content).unwrap();
        if let Err(e) = client.store(&msg.index, &value).await {
            eprintln!("[elastic][error]: {}", e)
        }
    }
}
