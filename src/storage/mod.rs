mod elastic_client;
mod fs_storage;

pub use fs_storage::FsStorage;

pub trait StorageStore {
    async fn store(
        &mut self,
        id: &String,
        data: &crate::core::Data,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
