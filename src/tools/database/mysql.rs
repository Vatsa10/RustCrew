use async_trait::async_trait;
use crate::tools::Tool;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Row, Column};

#[derive(Debug)]
pub struct MySqlTool {
    url: String,
}

impl MySqlTool {
    pub fn new(url: &str) -> Self {
        Self { url: url.to_string() }
    }
}

#[async_trait]
impl Tool for MySqlTool {
    fn name(&self) -> &str {
        "mysql"
    }

    fn description(&self) -> &str {
        "Query a MySQL database for data retrieval."
    }

    async fn execute(&self, query_str: &str) -> Result<String, String> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&self.url)
            .await
            .map_err(|e| e.to_string())?;

        let rows = sqlx::query(query_str)
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for row in rows {
            let mut row_data = Vec::new();
            for column in row.columns() {
                let name = column.name();
                let val: String = row.try_get_unchecked(name).unwrap_or_else(|_| "???".to_string());
                row_data.push(format!("{}: {}", name, val));
            }
            results.push(row_data.join(", "));
        }
        Ok(results.join("\n"))
    }
}
