use std::fs::{File, OpenOptions};
use std::io::Write;

use super::BlockchainStore;

pub struct FsBlockchain {
    file: File,
}

impl Default for FsBlockchain {
    fn default() -> Self {
        let path = "blockchain.txt";
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();

        FsBlockchain { file }
    }
}

impl BlockchainStore for FsBlockchain {
    async fn store(
        &mut self,
        id: &String,
        fingerprint: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.file, "{id} {fingerprint}")?;
        Ok(())
    }
}
