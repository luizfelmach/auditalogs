use crate::core::Data;

pub mod rabbitmq_client;
pub mod rust_client;

pub trait Queue {
    async fn on_message<F>(&self, callback: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Data) -> Result<(), Box<dyn std::error::Error>>;
}
