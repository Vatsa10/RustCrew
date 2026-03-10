use async_trait::async_trait;
use crate::memory::MemoryProvider;
use crate::memory::vector::VectorMemory;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct ContextualMemory {
    vector_store: Arc<VectorMemory>,
}

impl ContextualMemory {
    pub fn new(vector_store: Arc<VectorMemory>) -> Self {
        Self { vector_store }
    }

    pub async fn add_interaction(&self, agent: &str, interaction: &str) -> Result<(), String> {
        let timestamp = chrono::Utc::now().to_rfc3339();
        self.vector_store.store(
            &format!("ctx:{}", timestamp), 
            &format!("[{}] {}", agent, interaction)
        ).await
    }
}

#[async_trait]
impl MemoryProvider for ContextualMemory {
    async fn store(&self, key: &str, value: &str) -> Result<(), String> {
        self.vector_store.store(key, value).await
    }

    async fn retrieve(&self, key: &str) -> Result<Option<String>, String> {
        self.vector_store.retrieve(key).await
    }

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        self.vector_store.search(query, limit).await
    }
}
