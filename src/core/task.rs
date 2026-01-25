use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub expected_output: String,
    pub assigned_agent_id: Option<Uuid>,
    pub dependencies: Vec<Uuid>,
    pub status: TaskStatus,
    pub output: Option<String>,
}

impl Task {
    pub fn new(description: &str, expected_output: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.to_string(),
            expected_output: expected_output.to_string(),
            assigned_agent_id: None,
            dependencies: Vec::new(),
            status: TaskStatus::Pending,
            output: None,
        }
    }

    pub fn assign_agent(mut self, agent_id: Uuid) -> Self {
        self.assigned_agent_id = Some(agent_id);
        self
    }

    pub fn add_dependency(mut self, task_id: Uuid) -> Self {
        self.dependencies.push(task_id);
        self
    }
}
