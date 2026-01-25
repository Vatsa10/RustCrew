use sqlx::sqlite::SqlitePool;
use crate::core::task::{Task, TaskStatus};
use uuid::Uuid;

pub struct SqlMemory {
    pool: SqlitePool,
}

impl SqlMemory {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Simple manual migration for now
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                description TEXT NOT NULL,
                expected_output TEXT NOT NULL,
                assigned_agent_id TEXT,
                status TEXT NOT NULL,
                output TEXT
            )"
        ).execute(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn save_task(&self, task: &Task) -> Result<(), sqlx::Error> {
        let status = serde_json::to_string(&task.status).unwrap_or_default();
        let agent_id = task.assigned_agent_id.map(|id| id.to_string());
        
        sqlx::query(
            "INSERT OR REPLACE INTO tasks (id, description, expected_output, assigned_agent_id, status, output)
             VALUES (?, ?, ?, ?, ?, ?)"
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

    pub async fn get_task(&self, id: Uuid) -> Result<Option<Task>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT * FROM tasks WHERE id = ?",
            id.to_string()
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let status: TaskStatus = serde_json::from_str(&row.status).unwrap_or(TaskStatus::Pending);
            Ok(Some(Task {
                id: Uuid::parse_str(&row.id).unwrap_or_default(),
                description: row.description,
                expected_output: row.expected_output,
                assigned_agent_id: row.assigned_agent_id.and_then(|s| Uuid::parse_str(&s).ok()),
                dependencies: Vec::new(), // TODO: Store dependencies in a separate table
                status,
                output: row.output,
            }))
        } else {
            Ok(None)
        }
    }
}
