use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Vote {
    Approve,
    Reject,
    Abstain,
}

#[derive(Debug, Clone)]
pub struct ConsensusSession {
    pub id: Uuid,
    pub goal: String,
    pub votes: HashMap<Uuid, Vote>,
    pub required_votes: usize,
}

impl ConsensusSession {
    pub fn new(goal: &str, total_agents: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            goal: goal.to_string(),
            votes: HashMap::new(),
            required_votes: (total_agents / 2) + 1,
        }
    }

    pub fn cast_vote(&mut self, agent_id: Uuid, vote: Vote) {
        self.votes.insert(agent_id, vote);
    }

    pub fn is_reached(&self) -> bool {
        let approves = self.votes.values().filter(|v| matches!(v, Vote::Approve)).count();
        approves >= self.required_votes
    }

    pub fn is_rejected(&self) -> bool {
        let rejects = self.votes.values().filter(|v| matches!(v, Vote::Reject)).count();
        rejects >= self.required_votes
    }
}

#[async_trait::async_trait]
pub trait Critic: Send + Sync {
    async fn critique(&self, task_output: &str) -> Result<String, String>;
}

use std::sync::Arc;
use crate::core::llm::LlmAdapter;

pub struct VerifierAgent {
    pub instructions: String,
    pub llm: Arc<dyn LlmAdapter>,
}

impl VerifierAgent {
    pub fn new(instructions: &str, llm: Arc<dyn LlmAdapter>) -> Self {
        Self {
            instructions: instructions.to_string(),
            llm,
        }
    }
}

#[async_trait::async_trait]
impl Critic for VerifierAgent {
    async fn critique(&self, task_output: &str) -> Result<String, String> {
        let prompt = format!(
            "Evaluate the following output based on these instructions:\nInstructions: {}\nOutput: {}\nPlease provide a critique and determine if it's valid.",
            self.instructions, task_output
        );
        self.llm.completion(&prompt).await
    }
}
