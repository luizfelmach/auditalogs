use core::fmt;

use futures_lite::StreamExt;
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
    Connection, ConnectionProperties, Consumer, Result as LapinResult,
};

#[derive(Debug)]
pub enum ProcessorError {
    InvalidData(Option<String>),
}

impl fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessorError::InvalidData(Some(str)) => write!(f, "invalid data to processor: {str}"),
            ProcessorError::InvalidData(None) => write!(f, "invalid data to processor"),
        }
    }
}

#[derive(Debug)]
pub struct Rabbitmq {
    consumer: Consumer,
    batch: usize,
}

impl Rabbitmq {
    pub async fn new(uri: String, queue: String, batch: usize) -> LapinResult<Self> {
        let connection = Connection::connect(&uri, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        let consumer = channel
            .basic_consume(
                &queue,
                "consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        Ok(Rabbitmq { consumer, batch })
    }

    pub async fn consumer<F>(&mut self, processor: F)
    where
        F: Fn(Vec<Vec<u8>>) -> Result<(), ProcessorError>,
    {
        let mut batching: Vec<Delivery> = Vec::new();

        while let Some(delivery) = self.consumer.next().await {
            if let Ok(delivery) = delivery {
                batching.push(delivery);

                if batching.len() >= self.batch {
                    let mut messages = Vec::new();

                    for delivery in &batching {
                        messages.push(delivery.data.clone());
                    }

                    if let Err(error) = processor(messages) {
                        println!("error to processor messages: {}", error);
                        batching.clear();
                        continue;
                    }

                    for delivery in &batching {
                        delivery
                            .ack(BasicAckOptions::default())
                            .await
                            .expect("error on ack");
                    }

                    batching.clear();
                }
            }
        }
    }
}
