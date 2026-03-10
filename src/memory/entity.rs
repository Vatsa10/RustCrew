use async_trait::async_trait;
use crate::memory::MemoryProvider;
use crate::memory::sql::SqlMemory;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct EntityMemory {
    storage: Arc<SqlMemory>,
}

impl EntityMemory {
    pub fn new(storage: Arc<SqlMemory>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl MemoryProvider for EntityMemory {
    async fn store(&self, key: &str, value: &str) -> Result<(), String> {
        self.storage.store(&format!("entity:{}", key), value).await
    }

    async fn retrieve(&self, key: &str) -> Result<Option<String>, String> {
        self.storage.retrieve(&format!("entity:{}", key)).await
    }

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        // Simple search in the underlying SQL store
        self.storage.search(query, limit).await
    }
}
