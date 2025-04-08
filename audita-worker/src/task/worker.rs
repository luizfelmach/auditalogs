use crate::channel::{ElasticChannelItem, EthereumChannelItem, RxChannel, TxChannel};

pub async fn worker(tx: TxChannel, rx: RxChannel) {
    while let Some(msg) = rx.worker.lock().await.recv().await {
        println!("[worker] received message: {:?}", msg);

        let item = ElasticChannelItem {
            index: "OI".into(),
            content: "[]".into(),
        };
        let _ = tx.elastic.send(item).await.unwrap();

        let item = EthereumChannelItem {
            index: "OI".into(),
            hash: "0x97f978e380ff77d2be7f9735e353e1e417b00a080856fcffced3dd94bb1fa37f"
                .parse()
                .unwrap(),
        };
        let _ = tx.ethereum.send(item).await.unwrap();
    }
}
