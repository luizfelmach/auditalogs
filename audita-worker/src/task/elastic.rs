use crate::channel::{RxChannel, TxChannel};

pub async fn elastic(_: TxChannel, rx: RxChannel) {
    while let Some(msg) = rx.elastic.lock().await.recv().await {
        println!("[elastic] received message: {:?}", msg);
    }
}
