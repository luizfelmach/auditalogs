use crate::entity::document::Document;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storable {
    pub id: String,
    pub ord: usize,
    pub doc: Document,
}
