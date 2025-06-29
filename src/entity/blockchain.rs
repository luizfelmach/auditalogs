use crate::entity::hash::Hash256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingerprint {
    pub id: String,
    pub hash: Hash256,
}

impl Fingerprint {
    pub fn new(id: impl Into<String>, hash: Hash256) -> Self {
        Self {
            id: id.into(),
            hash,
        }
    }
}
