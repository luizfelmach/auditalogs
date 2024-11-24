use crate::core::Data;
pub mod elastic_client;

pub trait StorageStore {
    async fn store(&self, id: String, data: Data) -> Result<(), Box<dyn std::error::Error>>;
}
