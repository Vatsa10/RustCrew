use crate::core::task::Task;
use crate::core::llm::LlmAdapter;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub struct PlannerAgent {
    pub llm: Arc<dyn LlmAdapter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskPlan {
    pub description: String,
    pub expected_output: String,
}

impl PlannerAgent {
    pub fn new(llm: Arc<dyn LlmAdapter>) -> Self {
        Self { llm }
    }

    pub async fn create_plan(&self, goal: &str) -> Result<Vec<Task>, String> {
        let prompt = format!(
            "You are an expert project planner. Break down the following goal into a sequence of actionable tasks:\n\
             GOAL: {}\n\
             Output strictly a JSON array of objects. Each object must have a 'description' and 'expected_output' field.",
            goal
        );

        let response = self.llm.completion(&prompt).await?;
        // Clean markdown block if present
        let clean_json = response.trim_start_matches("```json").trim_end_matches("```").trim();

        let plans: Vec<TaskPlan> = serde_json::from_str(clean_json)
            .map_err(|e| format!("Failed to parse LLM planner response: {}. Re-try with clearer instructions.", e))?;

        let tasks: Vec<Task> = plans.into_iter()
            .map(|plan| Task::new(&plan.description, &plan.expected_output))
            .collect();

        Ok(tasks)
    }
}
