use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono;
use crate::core::agent::Agent;
use crate::core::task::Task;
use crate::memory::MemoryProvider;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub task_id: Uuid,
    pub event_type: String, // e.g., "started", "completed", "failed", "tool_call"
    pub data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CrewStatus {
    Idle,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub from: Uuid,
    pub to: Uuid,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct Crew {
    pub id: Uuid,
    pub agents: Vec<Agent>,
    pub tasks: Vec<Uuid>,
    pub task_map: HashMap<Uuid, Task>,
    pub memory: Option<Arc<dyn MemoryProvider>>,
    pub execution_trace: Vec<ExecutionEvent>,
    pub status: CrewStatus,
    pub messages: Vec<Message>,
    pub blackboard: HashMap<String, String>,
    pub consensus_sessions: HashMap<Uuid, crate::core::collaboration::ConsensusSession>,
}

impl Crew {
    pub fn new(agents: Vec<Agent>) -> Self {
        Self {
            id: Uuid::new_v4(),
            agents,
            tasks: Vec::new(),
            task_map: HashMap::new(),
            memory: None,
            execution_trace: Vec::new(),
            status: CrewStatus::Idle,
            messages: Vec::new(),
            blackboard: HashMap::new(),
            consensus_sessions: HashMap::new(),
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

    pub fn send_message(&mut self, from: Uuid, to: Uuid, content: &str) {
        self.messages.push(Message {
            from,
            to,
            content: content.to_string(),
            timestamp: chrono::Utc::now(),
        });
    }

    pub fn update_blackboard(&mut self, key: &str, value: &str) {
        self.blackboard.insert(key.to_string(), value.to_string());
    }
}
