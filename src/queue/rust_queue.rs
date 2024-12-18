use super::{Acker, Message};
use core::time;
use futures::{task::Poll, Stream};
use rand::random;
use serde_json::json;
use std::{
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct RustQueue {}

impl Default for RustQueue {
    fn default() -> Self {
        RustQueue {}
    }
}

impl Acker for () {}

impl Stream for RustQueue {
    type Item = Message;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        thread::sleep(time::Duration::from_millis(300));
        let item = Message {
            data: faker_log(),
            acker: Box::new(()),
        };
        Poll::Ready(Some(item))
    }
}

fn faker_log() -> Vec<u8> {
    let ip = format!(
        "{}.{}.{}.{}",
        random::<u8>(),
        random::<u8>(),
        random::<u8>(),
        random::<u8>()
    );
    let mac = format!(
        "{}:{}:{}:{}:{}:{}",
        format!("{:02x}", random::<u8>()),
        format!("{:02x}", random::<u8>()),
        format!("{:02x}", random::<u8>()),
        format!("{:02x}", random::<u8>()),
        format!("{:02x}", random::<u8>()),
        format!("{:02x}", random::<u8>()),
    );
    let port = random::<u16>();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let faker = json!({
        "ip": ip,
        "port": port,
        "mac": mac,
        "timestamp": timestamp,
    });
    serde_json::to_vec(&faker).unwrap()
}
