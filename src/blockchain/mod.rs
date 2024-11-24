pub mod blockchain_file_client;
pub mod ethereum_client;

pub trait BlockchainStore {
    async fn store(
        &mut self,
        id: String,
        fingerprint: String,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
