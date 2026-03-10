use crate::core::crew::Crew;
use chrono;
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
            let mut crew = self.crew.lock().await;
            crew.status = crate::core::crew::CrewStatus::Running;
            crew.tasks.clone()
        };

        loop {
            // Check for cancellation
            if {
                let crew = self.crew.lock().await;
                crew.status == crate::core::crew::CrewStatus::Cancelled
            } {
                println!("Crew execution cancelled.");
                return Ok(());
            }

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
                let mut crew = self.crew.lock().await;
                let all_done = tasks_to_run.iter().all(|id| {
                    crew.task_map.get(id).map_or(false, |t| matches!(t.status, TaskStatus::Completed | TaskStatus::Failed(_) | TaskStatus::TimedOut))
                });
                if all_done {
                    crew.status = crate::core::crew::CrewStatus::Completed;
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            }
            
            pending_tasks.sort(); // Sort Uuids for deterministic execution order

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
        let (description, agent_id, memory, retry_policy, timeout) = {
            let mut crew_lock = crew.lock().await;
            let memory = crew_lock.memory.clone();
            
            if let Some(task) = crew_lock.task_map.get_mut(&task_id) {
                task.status = TaskStatus::Running;
                println!("Executing task: {}", task.description);
                
                let desc = task.description.clone();
                let agent_id = task.assigned_agent_id;
                let policy = task.retry_policy.clone();
                let timeout = task.timeout;
                
                crew_lock.execution_trace.push(crate::core::crew::ExecutionEvent {
                    timestamp: chrono::Utc::now(),
                    task_id,
                    event_type: "started".to_string(),
                    data: None,
                });
                
                (desc, agent_id, memory, policy, timeout)
            } else {
                return;
            }
        };

        let mut attempts = 0;
        let mut last_error = String::from("Unknown error");

        while attempts < retry_policy.max_attempts {
            attempts += 1;
            
            let llm = if let Some(id) = agent_id {
                let crew_lock = crew.lock().await;
                crew_lock.agents.iter()
                    .find(|a| a.id == id)
                    .and_then(|a| a.llm.clone())
            } else {
                None
            };

            let execution_result = if let Some(llm_adapter) = llm {
                if let Some(t) = timeout {
                    match tokio::time::timeout(t, llm_adapter.completion(&description)).await {
                        Ok(Ok(res)) => Ok(res),
                        Ok(Err(e)) => Err(e),
                        Err(_) => Err("Task timed out".to_string()),
                    }
                } else {
                    llm_adapter.completion(&description).await
                }
            } else {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                Ok(format!("Executed: {}", description))
            };

            match execution_result {
                Ok(output) => {
                    if let Some(mem) = memory {
                        let _ = mem.store(&task_id.to_string(), &output).await;
                    }

                    {
                        let mut crew_lock = crew.lock().await;
                        if let Some(task) = crew_lock.task_map.get_mut(&task_id) {
                            task.status = TaskStatus::Completed;
                            task.output = Some(output.clone());
                        }
                        crew_lock.execution_trace.push(crate::core::crew::ExecutionEvent {
                            timestamp: chrono::Utc::now(),
                            task_id,
                            event_type: "completed".to_string(),
                            data: Some(output),
                        });
                    }
                    return;
                }
                Err(e) => {
                    last_error = e.clone();
                    {
                        let mut crew_lock = crew.lock().await;
                        crew_lock.execution_trace.push(crate::core::crew::ExecutionEvent {
                            timestamp: chrono::Utc::now(),
                            task_id,
                            event_type: "failed_attempt".to_string(),
                            data: Some(e),
                        });
                    }
                    if attempts < retry_policy.max_attempts {
                        tokio::time::sleep(tokio::time::Duration::from_millis(retry_policy.backoff_ms)).await;
                    }
                }
            }
        }

        {
            let mut crew_lock = crew.lock().await;
            if let Some(task) = crew_lock.task_map.get_mut(&task_id) {
                task.status = if last_error.contains("timed out") {
                    TaskStatus::TimedOut
                } else {
                    TaskStatus::Failed(last_error.clone())
                };
            }
            crew_lock.execution_trace.push(crate::core::crew::ExecutionEvent {
                timestamp: chrono::Utc::now(),
                task_id,
                event_type: "final_failure".to_string(),
                data: Some(last_error),
            });
        }
    }
}
