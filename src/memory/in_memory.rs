use uuid::Uuid;
use crate::core::task::Task;
use dashmap::DashMap;
use std::sync::Arc;

pub struct InMemoryMemory {
    tasks: Arc<DashMap<Uuid, Task>>,
}

impl InMemoryMemory {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(DashMap::new()),
        }
    }
}
