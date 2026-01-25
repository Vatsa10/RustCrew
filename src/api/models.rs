use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCrewRequest {
    pub name: String,
    pub agents: Vec<AgentSpec>,
    pub tasks: Vec<TaskSpec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentSpec {
    pub name: String,
    pub role: String,
    pub goal: String,
    pub backstory: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSpec {
    pub description: String,
    pub expected_output: String,
    pub agent_index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrewResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunResponse {
    pub id: Uuid,
    pub status: String,
}
