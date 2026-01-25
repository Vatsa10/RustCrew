use async_trait::async_trait;
use crate::tools::Tool;
use reqwest::Client;
use std::fmt::Debug;

#[derive(Debug)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Tool for HttpClient {
    fn name(&self) -> &str {
        "http_client"
    }

    fn description(&self) -> &str {
        "A tool for making HTTP GET requests. Input should be a valid URL."
    }

    async fn execute(&self, input: &str) -> Result<String, String> {
        let response = self.client.get(input)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let body = response.text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        Ok(body)
    }
}
