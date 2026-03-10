use sqlx::AnyPool;
use sqlx::Row;
use crate::core::task::Task;
use crate::memory::MemoryProvider;
use async_trait::async_trait;
use std::fmt::Debug;

#[derive(Debug)]
pub struct SqlMemory {
    pool: AnyPool,
}

impl SqlMemory {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        sqlx::any::install_default_drivers();
        let pool = AnyPool::connect(database_url).await?;
        
        let schema_tasks = if database_url.starts_with("postgres") {
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                description TEXT NOT NULL,
                expected_output TEXT NOT NULL,
                assigned_agent_id TEXT,
                status TEXT NOT NULL,
                output TEXT
            )"
        } else {
             "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                description TEXT NOT NULL,
                expected_output TEXT NOT NULL,
                assigned_agent_id TEXT,
                status TEXT NOT NULL,
                output TEXT
            )"
        };

        sqlx::query(schema_tasks).execute(&pool).await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS memory (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )"
        ).execute(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn save_task(&self, task: &Task) -> Result<(), sqlx::Error> {
        let status = serde_json::to_string(&task.status).unwrap_or_default();
        let agent_id = task.assigned_agent_id.map(|id| id.to_string());
        
        // Use generic SQL that works for both
        sqlx::query(
            "INSERT INTO tasks (id, description, expected_output, assigned_agent_id, status, output)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET 
                description = EXCLUDED.description,
                expected_output = EXCLUDED.expected_output,
                assigned_agent_id = EXCLUDED.assigned_agent_id,
                status = EXCLUDED.status,
                output = EXCLUDED.output"
        )
        .bind(task.id.to_string())
        .bind(&task.description)
        .bind(&task.expected_output)
        .bind(agent_id)
        .bind(status)
        .bind(&task.output)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl MemoryProvider for SqlMemory {
    async fn store(&self, key: &str, value: &str) -> Result<(), String> {
        sqlx::query("INSERT INTO memory (key, value) VALUES ($1, $2) ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value")
            .bind(key)
            .bind(value)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn retrieve(&self, key: &str) -> Result<Option<String>, String> {
        let row: Option<sqlx::any::AnyRow> = sqlx::query("SELECT value FROM memory WHERE key = $1")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(row.map(|r| r.get::<String, _>(0)))
    }

    async fn search(&self, query: &str, _limit: usize) -> Result<Vec<String>, String> {
        let rows: Vec<sqlx::any::AnyRow> = sqlx::query("SELECT value FROM memory WHERE value LIKE $1")
            .bind(format!("%{}%", query))
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().map(|r| r.get::<String, _>(0)).collect())
    }
}
