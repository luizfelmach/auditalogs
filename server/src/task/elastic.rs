use crate::{
    channel::{RxChannel, TxChannel},
    client::elastic::ElasticClient,
    config::AppConfig,
};

pub async fn elastic(config: AppConfig, _: TxChannel, rx: RxChannel) {
    let elastic = config.elastic;
    let client = ElasticClient::new(elastic.url, elastic.username, elastic.password).unwrap();

    if elastic.disable {
        log::warn!("Module is disabled. Skipping messages from channel.");
    }

    while let Some(msg) = rx.elastic.lock().await.recv().await {
        if elastic.disable {
            continue;
        }

        let value = match serde_json::from_str(&msg.content) {
            Ok(v) => v,
            Err(err) => {
                log::warn!("Failed to parse JSON: {:?}", err);
                continue;
            }
        };

        if let Err(err) = client.store(&msg.index, &value).await {
            log::error!("Failed to store document: {:?}", err);
        }
    }
    log::warn!("Elastic channel closed. Exiting elastic task");
}
