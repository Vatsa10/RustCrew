use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::tools::Tool;
use crate::core::llm::LlmAdapter;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MemoryScope {
    Agent,
    Crew,
    Global,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub role: String,
    pub goal: String,
    pub backstory: String,
    pub tools: Vec<Arc<dyn Tool>>,
    pub memory_scope: MemoryScope,
    pub llm: Option<Arc<dyn LlmAdapter>>,
}

impl Agent {
    pub fn new(name: &str, role: &str, goal: &str, backstory: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            role: role.to_string(),
            goal: goal.to_string(),
            backstory: backstory.to_string(),
            tools: Vec::new(),
            memory_scope: MemoryScope::Agent,
            llm: None,
        }
    }

    pub fn add_tool(mut self, tool: Arc<dyn Tool>) -> Self {
        self.tools.push(tool);
        self
    }

    pub fn add_llm(mut self, llm: Arc<dyn LlmAdapter>) -> Self {
        self.llm = Some(llm);
        self
    }
}
