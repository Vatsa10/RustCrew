use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait LlmAdapter: Send + Sync + Debug {
    async fn completion(&self, prompt: &str) -> Result<String, String>;
}

#[derive(Debug)]
pub struct OpenAiAdapter {
    api_key: String,
    model: String,
}

impl OpenAiAdapter {
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}

#[async_trait]
impl LlmAdapter for OpenAiAdapter {
    async fn completion(&self, _prompt: &str) -> Result<String, String> {
        // TODO: Implement OpenAI API call using reqwest
        Ok("Mocked response from OpenAI".to_string())
    }
}

#[derive(Debug)]
pub struct GeminiAdapter {
    api_key: String,
    model: String,
}

impl GeminiAdapter {
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}

#[async_trait]
impl LlmAdapter for GeminiAdapter {
    async fn completion(&self, _prompt: &str) -> Result<String, String> {
        // TODO: Implement Gemini API call using reqwest
        Ok("Mocked response from Gemini".to_string())
    }
}
