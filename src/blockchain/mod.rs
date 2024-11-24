mod ethereum_blockchain;
mod fs_blockchain;

pub use fs_blockchain::FsBlockchain;

pub trait BlockchainStore {
    async fn store(
        &mut self,
        id: &String,
        fingerprint: &String,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
