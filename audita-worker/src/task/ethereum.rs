use crate::{
    channel::{RxChannel, TxChannel},
    client::ethereum::EthereumClient,
    config::AppConfig,
};

pub async fn ethereum(config: AppConfig, _: TxChannel, rx: RxChannel) {
    let ethereum = config.ethereum;
    let client =
        EthereumClient::new(ethereum.url, ethereum.contract, ethereum.private_key).unwrap();

    let mut buffer = Vec::new();

    while let Some(msg) = rx.ethereum.lock().await.recv().await {
        if ethereum.disable {
            continue;
        }

        buffer.push(msg);
        if buffer.len() >= config.ethereum_batch_size {
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

            println!("Processed {} with {} success txs!", buffer.len(), ok);
            buffer.clear();
        }
    }
}
