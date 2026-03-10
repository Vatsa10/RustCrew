use async_trait::async_trait;
use std::fmt::Debug;
use serde_json::json;
use reqwest::Client;

#[async_trait]
pub trait LlmAdapter: Send + Sync + Debug {
    async fn completion(&self, prompt: &str) -> Result<String, String>;
}

// ----------------------------------------------------------------------------
// OpenAI
// ----------------------------------------------------------------------------
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
    async fn completion(&self, prompt: &str) -> Result<String, String> {
        let client = Client::new();
        let body = json!({
            "model": self.model,
            "messages": [
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.7
        });

        let res = client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            return Err(format!("OpenAI API error: {:?}", res.text().await));
        }

        let json_res: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        
        if let Some(content) = json_res["choices"][0]["message"]["content"].as_str() {
            Ok(content.to_string())
        } else {
            Err("Failed to parse OpenAI response".to_string())
        }
    }
}

// ----------------------------------------------------------------------------
// Gemini
// ----------------------------------------------------------------------------
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
    async fn completion(&self, prompt: &str) -> Result<String, String> {
        let client = Client::new();
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", self.model, self.api_key);
        
        let body = json!({
            "contents": [{
                "parts": [{"text": prompt}]
            }]
        });

        let res = client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            return Err(format!("Gemini API error: {:?}", res.text().await));
        }

        let json_res: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        
        if let Some(content) = json_res["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            Ok(content.to_string())
        } else {
            Err("Failed to parse Gemini response".to_string())
        }
    }
}

// ----------------------------------------------------------------------------
// Anthropic
// ----------------------------------------------------------------------------
#[derive(Debug)]
pub struct AnthropicAdapter {
    api_key: String,
    model: String,
}

impl AnthropicAdapter {
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}

#[async_trait]
impl LlmAdapter for AnthropicAdapter {
    async fn completion(&self, prompt: &str) -> Result<String, String> {
        let client = Client::new();
        let body = json!({
            "model": self.model,
            "max_tokens": 1024,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        });

        let res = client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            return Err(format!("Anthropic API error: {:?}", res.text().await));
        }

        let json_res: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        
        if let Some(content) = json_res["content"][0]["text"].as_str() {
            Ok(content.to_string())
        } else {
            Err("Failed to parse Anthropic response".to_string())
        }
    }
}

// ----------------------------------------------------------------------------
// Ollama (Local)
// ----------------------------------------------------------------------------
#[derive(Debug)]
pub struct OllamaAdapter {
    base_url: String,
    model: String,
}

impl OllamaAdapter {
    pub fn new(base_url: String, model: String) -> Self {
        Self { base_url, model }
    }
}

#[async_trait]
impl LlmAdapter for OllamaAdapter {
    async fn completion(&self, prompt: &str) -> Result<String, String> {
        let client = Client::new();
        let url = format!("{}/api/generate", self.base_url);
        
        let body = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false
        });

        let res = client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            return Err(format!("Ollama API error: {:?}", res.text().await));
        }

        let json_res: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        
        if let Some(content) = json_res["response"].as_str() {
            Ok(content.to_string())
        } else {
            Err("Failed to parse Ollama response".to_string())
        }
    }
}
