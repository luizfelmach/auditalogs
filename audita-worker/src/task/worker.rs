use crate::{
    channel::{ElasticChannelItem, EthereumChannelItem, RxChannel, TxChannel},
    config::AppConfig,
    utils::{elastic_index, fingerprint},
};

pub async fn worker(config: AppConfig, tx: TxChannel, rx: RxChannel) {
    let mut counter = 0;
    let mut hash = String::new();
    let mut index = elastic_index(&config.name);

    while let Some(msg) = rx.worker.lock().await.recv().await {
        hash = fingerprint(&hash, &msg);
        counter += 1;

        let item = ElasticChannelItem::new(index.clone(), msg.clone());
        if let Err(err) = tx.elastic.send(item).await {
            eprintln!("Failed to enqueue message to elastic: {err}");
        }

        if counter >= config.batch_size {
            let item = EthereumChannelItem::new(index.clone(), hash.clone().parse().unwrap());
            if let Err(err) = tx.ethereum.send(item).await {
                eprintln!("Failed to enqueue message to ethereum: {err}")
            }
            counter = 0;
            hash.clear();
            index = elastic_index(&config.name);
        }
    }
}
