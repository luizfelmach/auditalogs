use crate::channel::{RxChannel, TxChannel};

pub async fn ethereum(_: TxChannel, rx: RxChannel) {
    while let Some(msg) = rx.ethereum.lock().await.recv().await {
        println!("[ethereum] received message: {:?}", msg);
    }
}
