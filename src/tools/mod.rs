pub mod http;
pub mod file;
pub mod web_search;
pub mod github;
pub mod interpreter;

use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait Tool: Send + Sync + Debug {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, input: &str) -> Result<String, String>;
}
