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
        let row = sqlx::query(
            "SELECT id, description, expected_output, assigned_agent_id, status, output FROM tasks WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            use sqlx::Row;
            let status_str: String = row.get("status");
            let status: TaskStatus = serde_json::from_str(&status_str).unwrap_or(TaskStatus::Pending);
            let agent_id_str: Option<String> = row.get("assigned_agent_id");
            
            Ok(Some(Task {
                id: Uuid::parse_str(row.get("id")).unwrap_or_default(),
                description: row.get("description"),
                expected_output: row.get("expected_output"),
                assigned_agent_id: agent_id_str.and_then(|s| Uuid::parse_str(&s).ok()),
                dependencies: Vec::new(),
                status,
                output: row.get("output"),
            }))
        } else {
            Ok(None)
        }
    }
}
