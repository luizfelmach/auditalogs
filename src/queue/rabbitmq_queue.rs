use super::{Acker, Message};
use futures::executor::block_on;
use futures::task::Poll;
use futures::Stream;
use lapin::message::Delivery;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use lapin::{Connection, ConnectionProperties, Consumer};
use std::error::Error;

pub struct RabbitmqQueue {
    uri: String,
    queue: String,
    consumer_tag: String,
    consumer: Option<Consumer>,
}

impl Default for RabbitmqQueue {
    fn default() -> Self {
        RabbitmqQueue {
            uri: "amqp://rabbit:changeme@localhost:5672".into(),
            consumer_tag: "consumer".into(),
            queue: "queue".into(),
            consumer: None,
        }
    }
}

impl RabbitmqQueue {
    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let connection = Connection::connect(&self.uri, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        let consumer = channel
            .basic_consume(
                &self.queue,
                &self.consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        self.consumer = Some(consumer);
        Ok(())
    }
}

impl Acker for Delivery {
    fn queue_ack(&self) -> Result<(), Box<dyn Error>> {
        let result: Result<(), Box<dyn Error>> = block_on(async {
            self.ack(BasicAckOptions::default()).await?;
            Ok(())
        });

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

impl Stream for RabbitmqQueue {
    type Item = Result<Message, Box<dyn Error>>;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();

        let consumer = match &mut this.consumer {
            Some(consumer) => consumer,
            None => panic!("Unexpected error"),
        };

        let item = futures::ready!(std::pin::Pin::new(consumer).poll_next(cx));

        let item = match item {
            Some(Ok(delivery)) => delivery,
            Some(Err(err)) => return Poll::Ready(Some(Err(Box::new(err)))),
            None => panic!("Unexpected error"),
        };

        let message = Message {
            data: item.data.clone(),
            acker: Box::new(item),
        };

        Poll::Ready(Some(Ok(message)))
    }
}
