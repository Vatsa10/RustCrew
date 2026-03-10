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

pub struct VerifierAgent {
    pub instructions: String,
}

#[async_trait::async_trait]
impl Critic for VerifierAgent {
    async fn critique(&self, _task_output: &str) -> Result<String, String> {
        // In a real implementation, this would call an LLM with 'instructions' and 'task_output'
        Ok(format!("Critique (using instructions: {}): Input seems valid.", self.instructions))
    }
}
