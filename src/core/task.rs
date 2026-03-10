use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    TimedOut,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self { max_attempts: 1, backoff_ms: 1000 }
    }
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
    pub retry_policy: RetryPolicy,
    pub timeout: Option<Duration>,
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
            retry_policy: RetryPolicy::default(),
            timeout: Some(Duration::from_secs(300)), // Default 5 mins
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

    pub fn with_retry(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}
