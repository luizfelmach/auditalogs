use elastic::Elastic;
use rabbitmq::Rabbitmq;

mod elastic;
mod hashing;
mod rabbitmq;

#[tokio::main]
async fn main() {
    let elastic = Elastic::new("http://localhost:9200", "elastic", "changeme");

    let mut rabbit = Rabbitmq::new("amqp://rabbit:changeme@localhost:5672", "queue", 100)
        .await
        .expect("error connecting to rabbitmq");

    rabbit
        .consumer(|messages: Vec<Vec<u8>>| {
            let hash = hashing::fingerprint(&messages);

            elastic.store_data(&hash, &messages).unwrap();

            Ok(())
        })
        .await;
}
