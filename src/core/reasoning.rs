use crate::tools::Tool;
use crate::core::llm::LlmAdapter;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::core::task::Task;

#[derive(Serialize, Deserialize)]
pub struct ToolCallRequest {
    pub tool_name: String,
    pub input: String,
    pub justification: String,
}

pub struct ReasoningEngine {
    pub llm: Arc<dyn LlmAdapter>,
}

impl ReasoningEngine {
    pub fn new(llm: Arc<dyn LlmAdapter>) -> Self {
        Self { llm }
    }

    pub async fn select_tool(&self, task_description: &str, available_tools: &[Arc<dyn Tool>]) -> Result<Option<ToolCallRequest>, String> {
        let mut tool_descriptions = String::new();
        for tool in available_tools {
            tool_descriptions.push_str(&format!("- {}: {}\n", tool.name(), tool.description()));
        }

        let prompt = format!(
            "Task: {}\n\nAvailable tools:\n{}\n\nWhich tool should be used next to accomplish the task? Output JSON with 'tool_name', 'input' (what to pass to the tool), and 'justification'. If no tool is needed, return empty JSON {{}}.",
            task_description, tool_descriptions
        );

        let response = self.llm.completion(&prompt).await?;
        let clean_json = response.trim_start_matches("```json").trim_end_matches("```").trim();
        
        if clean_json == "{}" || clean_json.is_empty() {
            return Ok(None);
        }

        let request: ToolCallRequest = serde_json::from_str(clean_json)
            .map_err(|e| format!("Failed to parse tool selection JSON: {}", e))?;

        Ok(Some(request))
    }

    pub async fn evaluate_execution(&self, expected_output: &str, actual_output: &str) -> Result<String, String> {
        let prompt = format!(
            "You are an evaluator agent. Check if the 'Actual Output' meets the criteria defined in the 'Expected Output'. Provide feedback on what is missing or if it's completely successful.\nExpected Output: {}\n\nActual Output: {}",
            expected_output, actual_output
        );
        self.llm.completion(&prompt).await
    }

    pub async fn propose_dynamic_tasks(&self, current_context: &str) -> Result<Vec<Task>, String> {
        let prompt = format!(
            "Based on the following execution context, what next steps are dynamically required to satisfy the overarching goal?\nContext: {}\nOutput a JSON array of objects with 'description' and 'expected_output'. Return an empty array if no further tasks are needed.",
            current_context
        );
        
        let response = self.llm.completion(&prompt).await?;
        let clean_json = response.trim_start_matches("```json").trim_end_matches("```").trim();
        
        if clean_json == "[]" || clean_json.is_empty() {
            return Ok(vec![]);
        }

        let plans: Vec<crate::core::planning::TaskPlan> = serde_json::from_str(clean_json)
            .map_err(|e| format!("Parsing error for dynamic tasks: {}", e))?;
            
        Ok(plans.into_iter().map(|p| Task::new(&p.description, &p.expected_output)).collect())
    }
}
