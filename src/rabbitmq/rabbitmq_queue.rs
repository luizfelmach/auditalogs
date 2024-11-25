use lapin::{
    options::BasicConsumeOptions, types::FieldTable, Connection, ConnectionProperties, Consumer,
};

pub struct RabbitmqQueue {
    uri: String,
    queue: String,
}

impl Default for RabbitmqQueue {
    fn default() -> Self {
        RabbitmqQueue {
            uri: "amqp://rabbit:changeme@localhost:5672".into(),
            queue: "queue".into(),
        }
    }
}

impl RabbitmqQueue {
    pub async fn consumer(&mut self) -> Result<Consumer, Box<dyn std::error::Error>> {
        let connection = Connection::connect(&self.uri, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        let consumer = channel
            .basic_consume(
                &self.queue,
                "consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        Ok(consumer)
    }
}

// impl Stream for RabbitmqQueue {
//     type Item = Data;

//     fn poll_next(
//         self: std::pin::Pin<&mut Self>,
//         _cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Option<Self::Item>> {
//         let this = self.get_mut();

//         let consumer = match &this.consumer {
//             None => panic!("lskdjf"),
//             Some(consumer) => consumer,
//         };

//         //let mut batching: Vec<Delivery> = Vec::new();
//     }
// }

// use futures_lite::StreamExt;
// use lapin::{
//     message::Delivery,
//     options::{BasicAckOptions, BasicConsumeOptions},
//     types::FieldTable,
//     Connection, ConnectionProperties, Consumer, Result as LapinResult,
// };
// use std::error::Error;
// use tracing::{event, info, warn, Level};

// #[derive(Debug)]
// pub struct Rabbitmq {
//     consumer: Consumer,
//     batch: usize,
// }

// impl Rabbitmq {
//     pub async fn new(uri: &str, queue: &str, batch: usize) -> LapinResult<Self> {
//         let connection = Connection::connect(uri, ConnectionProperties::default()).await?;
//         let channel = connection.create_channel().await?;
//         let consumer = channel
//             .basic_consume(
//                 queue,
//                 "consumer",
//                 BasicConsumeOptions::default(),
//                 FieldTable::default(),
//             )
//             .await?;

//         info!("Connected to RabbitMQ at {uri}. The message exchange has begun!");
//         Ok(Rabbitmq { consumer, batch })
//     }

//     pub async fn consumer<F>(&mut self, processor: F)
//     where
//         F: Fn(Vec<Vec<u8>>) -> Result<(), Box<dyn Error>>,
//     {
//         let mut batching: Vec<Delivery> = Vec::new();

//         info!("Waiting for messages in the queue. Ready to process!");

//         while let Some(delivery) = self.consumer.next().await {
//             if let Ok(delivery) = delivery {
//                 batching.push(delivery);

//                 if batching.len() >= self.batch {
//                     event!(
//                         Level::INFO,
//                         "Received {} messages in the queue. Ready to process.",
//                         batching.len()
//                     );

//                     let mut messages = Vec::new();

//                     for delivery in &batching {
//                         messages.push(delivery.data.clone());
//                     }

//                     if let Err(error) = processor(messages) {
//                         warn!("Failed to process messages: {error}");
//                         warn!("Returning them to the queue for retry.");
//                         batching.clear();
//                         continue;
//                     }

//                     for delivery in &batching {
//                         delivery
//                             .ack(BasicAckOptions::default())
//                             .await
//                             .expect("error on ack delivery");
//                     }

//                     batching.clear();
//                 }
//             }
//         }
//     }
// }
