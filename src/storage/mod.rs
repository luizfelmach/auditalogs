pub mod elasticsearch;
pub mod search;

use crate::{entity::Storable, storage::search::QueryExpr};
use anyhow::Result;

pub trait Storage {
    async fn store(&self, items: &Vec<Storable>) -> Result<()>;
    async fn retrieve(&self, batch_id: &str) -> Result<Vec<Storable>>;
    async fn search(&self, query: QueryExpr) -> Result<Vec<Storable>>;
}

pub trait SearchInterpreter<T> {
    fn interpret(&self, query: &QueryExpr) -> Result<T>;
}
