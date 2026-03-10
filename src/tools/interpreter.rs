use async_trait::async_trait;
use crate::tools::Tool;
use std::process::Command;

#[derive(Debug)]
pub struct CodeInterpreter;

#[async_trait]
impl Tool for CodeInterpreter {
    fn name(&self) -> &str {
        "interpreter"
    }

    fn description(&self) -> &str {
        "Execute local shell commands to perform computations or file tasks."
    }

    async fn execute(&self, command: &str) -> Result<String, String> {
        // Warning: This is NOT sandboxed. Use with caution.
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(command)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}
