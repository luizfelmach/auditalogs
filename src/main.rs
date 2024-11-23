use rabbitmq::Rabbitmq;
use uuid::Uuid;

mod hashing;
mod rabbitmq;

// fn save_on_chain(id: &String, hash: &String) -> Result<(), SaveError> {
//     Ok(())
// }

// fn save_off_chain(id: &String, hash: &String) -> Result<(), SaveError> {
//     Ok(())
// }

// fn procesor(data: Vec<Vec<u8>>) -> Result<(), SaveError> {
//     save_on_chain(&id, &hash).unwrap();
//     save_off_chain(&id, &hash).unwrap();
//     Ok(())
// }

#[tokio::main]
async fn main() {
    let mut client = Rabbitmq::new(
        "amqp://rabbit:changeme@localhost:5672".into(),
        "queue".into(),
        100,
    )
    .await
    .expect("error connecting to rabbitmq");

    client
        .consumer(|messages: Vec<Vec<u8>>| {
            let id = Uuid::new_v4().to_string();
            let hash = hashing::fingerprint(&messages);
            println!("{id} with {hash}");
            Ok(())
        })
        .await;
}
