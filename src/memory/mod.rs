pub mod in_memory;
pub mod sql;
pub mod short_term;
pub mod vector;
pub mod entity;
pub mod contextual;

use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::Arc;

#[async_trait]
pub trait MemoryProvider: Send + Sync + Debug {
    async fn store(&self, key: &str, value: &str) -> Result<(), String>;
    async fn retrieve(&self, key: &str) -> Result<Option<String>, String>;
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<String>, String>;
}

#[derive(Clone, Debug)]
pub struct MemorySystem {
    pub short_term: Arc<short_term::ShortTermMemory>,
    pub sql: Option<Arc<sql::SqlMemory>>,
    pub vector: Option<Arc<vector::VectorMemory>>,
    pub entity: Option<Arc<entity::EntityMemory>>,
    pub contextual: Option<Arc<contextual::ContextualMemory>>,
}

impl MemorySystem {
    pub fn new() -> Self {
        Self {
            short_term: Arc::new(short_term::ShortTermMemory::new()),
            sql: None,
            vector: None,
            entity: None,
            contextual: None,
        }
    }

    pub fn with_sql(mut self, sql: Arc<sql::SqlMemory>) -> Self {
        self.sql = Some(sql);
        self
    }

    pub fn with_vector(mut self, vector: Arc<vector::VectorMemory>) -> Self {
        self.vector = Some(vector);
        self
    }

    pub fn with_entity(mut self, entity: Arc<entity::EntityMemory>) -> Self {
        self.entity = Some(entity);
        self
    }

    pub fn with_contextual(mut self, contextual: Arc<contextual::ContextualMemory>) -> Self {
        self.contextual = Some(contextual);
        self
    }
}
