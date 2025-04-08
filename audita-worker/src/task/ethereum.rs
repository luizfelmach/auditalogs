use crate::{
    channel::{EthereumChannelItem, RxChannel, TxChannel},
    client::ethereum::EthereumClient,
    config::AppConfig,
};

const BATCH_ETHEREUM: usize = 1;

pub async fn ethereum(config: AppConfig, _: TxChannel, rx: RxChannel) {
    let mut buffer = Vec::new();

    while let Some(msg) = rx.ethereum.lock().await.recv().await {
        buffer.push(msg);
        if buffer.len() >= BATCH_ETHEREUM {
            let (tot, suc) = process(&buffer).await;
            println!("Processed {} with {} success txs!", tot, suc);
            buffer.clear();
        }
    }
}

async fn process(buffer: &Vec<EthereumChannelItem>) -> (usize, usize) {
    let client = EthereumClient::new(
        "http://127.0.0.1:8545".into(),
        "0x42699A7612A82f1d9C36148af9C77354759b210b".into(),
        "0x8f2a55949038a9610f50fb23b5883af3b4ecb3c3bb792cbcefbd1542c692be63".into(),
    )
    .await
    .unwrap();
    let mut nonce = client.nonce().await.unwrap();

    let mut txs = Vec::new();

    for content in buffer.iter() {
        let result = client.send_tx(nonce, &content.index, content.hash, 3).await;
        if let Ok(tx_hash) = result {
            txs.push((nonce, tx_hash));
        }
        nonce += 1;
    }

    let mut ok = 0;

    for (tx_nonce, tx_hash) in txs.iter() {
        let result = client.wait_for_tx(*tx_hash).await;
        match result {
            Ok(_) => ok += 1,
            Err(_) => {
                client.remove_tx(*tx_nonce).await.unwrap();
            }
        }
    }

    return (buffer.len(), ok);
}
