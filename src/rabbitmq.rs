use futures_lite::StreamExt;
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
    Connection, ConnectionProperties, Consumer, Result as LapinResult,
};
use std::error::Error;
use tracing::{event, info, warn, Level};

#[derive(Debug)]
pub struct Rabbitmq {
    consumer: Consumer,
    batch: usize,
}

impl Rabbitmq {
    pub async fn new(uri: &str, queue: &str, batch: usize) -> LapinResult<Self> {
        let connection = Connection::connect(uri, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        let consumer = channel
            .basic_consume(
                queue,
                "consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!("Connected to RabbitMQ at {uri}. The message exchange has begun!");
        Ok(Rabbitmq { consumer, batch })
    }

    pub async fn consumer<F>(&mut self, processor: F)
    where
        F: Fn(Vec<Vec<u8>>) -> Result<(), Box<dyn Error>>,
    {
        let mut batching: Vec<Delivery> = Vec::new();

        info!("Waiting for messages in the queue. Ready to process!");

        while let Some(delivery) = self.consumer.next().await {
            if let Ok(delivery) = delivery {
                batching.push(delivery);

                if batching.len() >= self.batch {
                    event!(
                        Level::INFO,
                        "Received {} messages in the queue. Ready to process.",
                        batching.len()
                    );

                    let mut messages = Vec::new();

                    for delivery in &batching {
                        messages.push(delivery.data.clone());
                    }

                    if let Err(error) = processor(messages) {
                        warn!("Failed to process messages: {error}");
                        warn!("Returning them to the queue for retry.");
                        batching.clear();
                        continue;
                    }

                    for delivery in &batching {
                        delivery
                            .ack(BasicAckOptions::default())
                            .await
                            .expect("error on ack delivery");
                    }

                    batching.clear();
                }
            }
        }
    }
}
