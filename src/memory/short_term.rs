use std::collections::HashMap;
use tokio::sync::RwLock;
use async_trait::async_trait;
use crate::memory::MemoryProvider;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ShortTermMemory {
    storage: RwLock<HashMap<String, String>>,
}

impl ShortTermMemory {
    pub fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl MemoryProvider for ShortTermMemory {
    async fn store(&self, key: &str, value: &str) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn retrieve(&self, key: &str) -> Result<Option<String>, String> {
        let storage = self.storage.read().await;
        Ok(storage.get(key).cloned())
    }

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let storage = self.storage.read().await;
        let results: Vec<String> = storage
            .values()
            .filter(|v| v.contains(query))
            .take(limit)
            .cloned()
            .collect();
        Ok(results)
    }
}
