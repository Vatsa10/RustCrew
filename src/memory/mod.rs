pub mod in_memory;
pub mod sql;
pub mod short_term;
pub mod vector;
pub mod entity;
pub mod contextual;

use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait MemoryProvider: Send + Sync + Debug {
    async fn store(&self, key: &str, value: &str) -> Result<(), String>;
    async fn retrieve(&self, key: &str) -> Result<Option<String>, String>;
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<String>, String>;
}
