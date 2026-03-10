use uuid::Uuid;
use crate::core::agent::Agent;
use crate::core::task::Task;
use crate::memory::MemoryProvider;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Crew {
    pub id: Uuid,
    pub agents: Vec<Agent>,
    pub tasks: Vec<Uuid>,
    pub task_map: HashMap<Uuid, Task>,
    pub memory: Option<Arc<dyn MemoryProvider>>,
}

impl Crew {
    pub fn new(agents: Vec<Agent>) -> Self {
        Self {
            id: Uuid::new_v4(),
            agents,
            tasks: Vec::new(),
            task_map: HashMap::new(),
            memory: None,
        }
    }

    pub fn with_memory(mut self, memory: Arc<dyn MemoryProvider>) -> Self {
        self.memory = Some(memory);
        self
    }

    pub fn add_task(&mut self, task: Task) {
        let id = task.id;
        self.task_map.insert(id, task);
        self.tasks.push(id);
    }
}
