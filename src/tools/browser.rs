use async_trait::async_trait;
use crate::tools::Tool;
use headless_chrome::Browser;

#[derive(Debug)]
pub struct BrowserTool;

impl BrowserTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for BrowserTool {
    fn name(&self) -> &str {
        "browser"
    }

    fn description(&self) -> &str {
        "Automates a headless browser to open URLs and extract text content."
    }

    async fn execute(&self, url: &str) -> Result<String, String> {
        let url_owned = url.to_string();
        tokio::task::spawn_blocking(move || {
            let browser = Browser::default().map_err(|e| e.to_string())?;
            let tab = browser.new_tab().map_err(|e| e.to_string())?;
            tab.navigate_to(&url_owned).map_err(|e| e.to_string())?;
            tab.wait_until_navigated().map_err(|e| e.to_string())?;
            let content = tab.get_content().map_err(|e| e.to_string())?;
            Ok(content)
        }).await.map_err(|e| e.to_string())?
    }
}
