use crate::core::crew::Crew;
use crate::core::task::TaskStatus;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashSet;
use uuid::Uuid;
use futures::stream::{StreamExt, FuturesUnordered};

pub struct Scheduler {
    pub crew: Arc<Mutex<Crew>>,
}

impl Scheduler {
    pub fn new(crew: Crew) -> Self {
        Self {
            crew: Arc::new(Mutex::new(crew)),
        }
    }

    pub async fn run(&self) -> Result<(), String> {
        let mut completed_tasks = HashSet::new();
        let tasks_to_run = {
            let crew = self.crew.lock().await;
            crew.tasks.clone()
        };

        loop {
            let mut pending_tasks = Vec::new();
            {
                let crew = self.crew.lock().await;
                for &task_id in &tasks_to_run {
                    if completed_tasks.contains(&task_id) {
                        continue;
                    }

                    if let Some(task) = crew.task_map.get(&task_id) {
                        if task.status == TaskStatus::Pending {
                            // Check if all dependencies are satisfied
                            let deps_satisfied = task.dependencies.iter().all(|dep_id| completed_tasks.contains(dep_id));
                            if deps_satisfied {
                                pending_tasks.push(task_id);
                            }
                        }
                    }
                }
            }

            if pending_tasks.is_empty() {
                let crew = self.crew.lock().await;
                let all_done = tasks_to_run.iter().all(|id| {
                    crew.task_map.get(id).map_or(false, |t| matches!(t.status, TaskStatus::Completed | TaskStatus::Failed(_)))
                });
                if all_done {
                    break;
                }
                // If not all done but no pending, we might have a deadlock or waiting for async tasks
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            }

            let mut futures = FuturesUnordered::new();
            for task_id in pending_tasks {
                let crew_ref = Arc::clone(&self.crew);
                futures.push(async move {
                    Self::execute_task(crew_ref, task_id).await;
                    task_id
                });
            }

            while let Some(task_id) = futures.next().await {
                completed_tasks.insert(task_id);
            }
        }

        println!("All tasks in crew completed.");
        Ok(())
    }

    async fn execute_task(crew: Arc<Mutex<Crew>>, task_id: Uuid) {
        let (description, _agent_id) = {
            let mut crew_lock = crew.lock().await;
            if let Some(task) = crew_lock.task_map.get_mut(&task_id) {
                task.status = TaskStatus::Running;
                println!("Executing task: {}", task.description);
                (task.description.clone(), task.assigned_agent_id)
            } else {
                return;
            }
        };

        // Simulate execution
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let output = format!("Executed: {}", description);

        {
            let mut crew_lock = crew.lock().await;
            if let Some(task) = crew_lock.task_map.get_mut(&task_id) {
                task.status = TaskStatus::Completed;
                task.output = Some(output);
            }
        }
    }
}
