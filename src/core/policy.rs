use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ToolPolicy {
    pub allowed_tools: HashSet<String>,
    pub blocked_keywords: Vec<String>,
}

impl ToolPolicy {
    pub fn new() -> Self {
        Self {
            allowed_tools: HashSet::new(),
            blocked_keywords: Vec::new(),
        }
    }

    pub fn is_allowed(&self, tool_name: &str, input: &str) -> bool {
        if !self.allowed_tools.is_empty() && !self.allowed_tools.contains(tool_name) {
            return false;
        }

        for keyword in &self.blocked_keywords {
            if input.contains(keyword) {
                return false;
            }
        }

        true
    }
}
