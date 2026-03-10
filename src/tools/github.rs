use async_trait::async_trait;
use crate::tools::Tool;
use serde_json::Value;

#[derive(Debug)]
pub struct GitHubTool {
    auth_token: Option<String>,
}

impl GitHubTool {
    pub fn new(auth_token: Option<String>) -> Self {
        Self { auth_token }
    }
}

#[async_trait]
impl Tool for GitHubTool {
    fn name(&self) -> &str {
        "github"
    }

    fn description(&self) -> &str {
        "Interact with GitHub to fetch info about repos, issues, etc."
    }

    async fn execute(&self, repo_path: &str) -> Result<String, String> {
        let client = reqwest::Client::new();
        let mut request = client.get(format!("https://api.github.com/repos/{}", repo_path))
            .header("User-Agent", "RustCrew-Agent");

        if let Some(token) = &self.auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let res = request.send().await.map_err(|e| e.to_string())?;
        let json: Value = res.json().await.map_err(|e| e.to_string())?;

        if let Some(msg) = json.get("message") {
            return Err(msg.as_str().unwrap_or("Unknown GitHub error").to_string());
        }

        let full_name = json.get("full_name").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let description = json.get("description").and_then(|v| v.as_str()).unwrap_or("No description");
        let stars = json.get("stargazers_count").and_then(|v| v.as_i64()).unwrap_or(0);
        let language = json.get("language").and_then(|v| v.as_str()).unwrap_or("Unknown");

        Ok(format!("Repo: {}\nDescription: {}\nStars: {}\nLanguage: {}", 
            full_name, description, stars, language))
    }
}
