use async_trait::async_trait;
use crate::memory::MemoryProvider;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ContextualMemory {
    // Focuses on interaction context
}

impl ContextualMemory {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MemoryProvider for ContextualMemory {
    async fn store(&self, _key: &str, _value: &str) -> Result<(), String> {
        Ok(())
    }

    async fn retrieve(&self, _key: &str) -> Result<Option<String>, String> {
        Ok(None)
    }

    async fn search(&self, _query: &str, _limit: usize) -> Result<Vec<String>, String> {
        Ok(vec![])
    }
}
