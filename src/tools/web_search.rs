use async_trait::async_trait;
use crate::tools::Tool;
use serde_json::Value;

#[derive(Debug)]
pub struct WebSearch {
    api_key: String,
}

impl WebSearch {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl Tool for WebSearch {
    fn name(&self) -> &str {
        "web_search"
    }

    fn description(&self) -> &str {
        "Search the web using Serper API to get up-to-date information."
    }

    async fn execute(&self, query: &str) -> Result<String, String> {
        let client = reqwest::Client::new();
        let body = serde_json::json!({
            "q": query
        });

        let res = client.post("https://google.serper.dev/search")
            .header("X-API-KEY", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: Value = res.json().await.map_err(|e| e.to_string())?;
        
        // Extract top snippets
        let mut results = Vec::new();
        if let Some(organic) = json.get("organic").and_then(|v| v.as_array()) {
            for item in organic.iter().take(5) {
                if let (Some(title), Some(snippet)) = (item.get("title"), item.get("snippet")) {
                    results.push(format!("{}: {}", title, snippet));
                }
            }
        }

        if results.is_empty() {
            Ok("No organic search results found.".to_string())
        } else {
            Ok(results.join("\n\n"))
        }
    }
}
