use async_trait::async_trait;
use crate::tools::Tool;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Row, Column};

#[derive(Debug)]
pub struct PostgresTool {
    pool: Option<sqlx::PgPool>,
    url: String,
}

impl PostgresTool {
    pub fn new(url: &str) -> Self {
        Self { pool: None, url: url.to_string() }
    }
}

#[async_trait]
impl Tool for PostgresTool {
    fn name(&self) -> &str {
        "postgres"
    }

    fn description(&self) -> &str {
        "Execute standard and vector (pgvector) SQL queries on a PostgreSQL database."
    }

    async fn execute(&self, query_str: &str) -> Result<String, String> {
        let pool = PgPoolOptions::new()
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
            // Collect arbitrary rows into a string format
            let mut row_data = Vec::new();
            for column in row.columns() {
                let name = column.name();
                // Simple textualization of data
                let val: String = row.try_get_unchecked(name).unwrap_or_else(|_| "???".to_string());
                row_data.push(format!("{}: {}", name, val));
            }
            results.push(row_data.join(", "));
        }

        Ok(results.join("\n"))
    }
}
