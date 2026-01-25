use async_trait::async_trait;
use crate::tools::Tool;
use tokio::fs;
use std::fmt::Debug;
use std::path::Path;

#[derive(Debug)]
pub struct FileLoader;

impl FileLoader {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for FileLoader {
    fn name(&self) -> &str {
        "file_loader"
    }

    fn description(&self) -> &str {
        "A tool for loading content from a file. Input should be a valid file path."
    }

    async fn execute(&self, input: &str) -> Result<String, String> {
        let path = Path::new(input);
        if !path.exists() {
            return Err(format!("File does not exist: {}", input));
        }

        let content = fs::read_to_string(path)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))?;

        Ok(content)
    }
}
