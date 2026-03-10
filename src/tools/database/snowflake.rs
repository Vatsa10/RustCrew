use async_trait::async_trait;
use crate::tools::Tool;
use serde_json::json;

#[derive(Debug)]
pub struct SnowflakeTool {
    account: String,
    token: String,
}

impl SnowflakeTool {
    pub fn new(account: &str, token: &str) -> Self {
        Self { 
            account: account.to_string(), 
            token: token.to_string() 
        }
    }
}

#[async_trait]
impl Tool for SnowflakeTool {
    fn name(&self) -> &str {
        "snowflake"
    }

    fn description(&self) -> &str {
        "Integrate with Snowflake data warehouses via the REST SQL API."
    }

    async fn execute(&self, sql_statement: &str) -> Result<String, String> {
        let client = reqwest::Client::new();
        let url = format!("https://{}.snowflakecomputing.com/api/v2/statements", self.account);
        
        let body = json!({
            "statement": sql_statement
        });

        let res = client.post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("X-Snowflake-Authorization-Token-Type", "OAUTH")
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let text = res.text().await.map_err(|e| e.to_string())?;
        Ok(text)
    }
}
