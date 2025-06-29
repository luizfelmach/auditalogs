use crate::entity::{Document, Hash256};
use anyhow::{Ok, Result};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug)]
pub struct Batch {
    pub id: String,
    pub hash: Hash256,
    pub count: usize,
}

impl Batch {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4().to_string(), hash: Hash256::zero(), count: 0 }
    }

    pub fn add(&mut self, data: &Document) -> Result<()> {
        let mut hasher = Sha256::new();
        hasher.update(self.hash);
        hasher.update(serde_json::to_vec(data)?);
        let result = hasher.finalize().into();
        self.hash = Hash256::from_slice(&result);
        self.count += 1;
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
