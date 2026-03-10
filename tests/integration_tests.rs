use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use std::time::Duration;

use rustcrew::core::agent::{Agent, MemoryScope};
use rustcrew::core::crew::{Crew, CrewStatus};
use rustcrew::core::scheduler::Scheduler;
use rustcrew::core::task::{Task, TaskStatus, RetryPolicy};
// Assuming we have a mock llm adapter for tests:
// If it's private to `rustcrew`, we either need it to be public or implement one locally.
use rustcrew::core::llm::{LlmAdapter, OpenAiAdapter};

#[tokio::test]
async fn test_end_to_end_crew_execution() {
    let agent = Agent::new("TestAgent", "Tester", "Run basic tests", "A senior testing engineer");
    let task = Task::new("Write a simple string", "The string 'hello world'").assign_agent(agent.id);
    
    let mut crew = Crew::new(vec![agent]);
    crew.add_task(task);
    
    let crew_arc = Arc::new(Mutex::new(crew));
    let scheduler = Scheduler { crew: crew_arc.clone() };
    
    let result = scheduler.run().await;
    assert!(result.is_ok(), "Scheduler run should complete successfully.");
    
    let crew_lock = crew_arc.lock().await;
    assert_eq!(crew_lock.status, CrewStatus::Completed, "Crew should be marked as completed.");
    
    let task_id = crew_lock.tasks[0];
    let executed_task = crew_lock.task_map.get(&task_id).unwrap();
    assert_eq!(executed_task.status, TaskStatus::Completed, "Task should be completed.");
    assert!(executed_task.output.is_some(), "Task should have generated output.");
}

#[tokio::test]
async fn test_task_dependencies_and_determinism() {
    let agent = Agent::new("A1", "Worker", "Do work", "Worker");
    
    let task1 = Task::new("Task 1", "Output 1").assign_agent(agent.id);
    let mut task2 = Task::new("Task 2", "Output 2").assign_agent(agent.id);
    task2.dependencies.push(task1.id); // task2 depends on task1
    
    let t1_id = task1.id;
    let t2_id = task2.id;

    let mut crew = Crew::new(vec![agent]);
    crew.add_task(task1);
    crew.add_task(task2);
    
    let crew_arc = Arc::new(Mutex::new(crew));
    let scheduler = Scheduler { crew: crew_arc.clone() };
    
    let result = scheduler.run().await;
    assert!(result.is_ok());
    
    let crew_lock = crew_arc.lock().await;
    assert_eq!(crew_lock.status, CrewStatus::Completed);
    
    // Check trace for execution order
    let trace = &crew_lock.execution_trace;
    let mut t1_completed_index = 0;
    let mut t2_started_index = 0;
    
    for (i, event) in trace.iter().enumerate() {
        if event.task_id == t1_id && event.event_type == "completed" {
            t1_completed_index = i;
        }
        if event.task_id == t2_id && event.event_type == "started" {
            t2_started_index = i;
        }
    }
    
    assert!(t1_completed_index < t2_started_index, "Task 1 must complete before Task 2 starts.");
}

#[tokio::test]
async fn test_cancellation() {
    let agent = Agent::new("A1", "Worker", "Do work", "Worker");
    let task = Task::new("Long task", "taking long").assign_agent(agent.id);
    
    let mut crew = Crew::new(vec![agent]);
    crew.add_task(task);
    let crew_arc = Arc::new(Mutex::new(crew));
    let scheduler = Scheduler { crew: crew_arc.clone() };
    
    let crew_arc_clone = crew_arc.clone();
    tokio::spawn(async move {
        // Cancel the crew slightly after starting
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        let mut c = crew_arc_clone.lock().await;
        c.status = CrewStatus::Cancelled;
    });

    let _ = scheduler.run().await;
    
    let c = crew_arc.lock().await;
    assert_eq!(c.status, CrewStatus::Cancelled, "Crew should be cancelled.");
}

#[test]
fn test_task_creation() {
    let task = Task::new("Desc", "Out");
    assert_eq!(task.description, "Desc");
    assert_eq!(task.expected_output, "Out");
    assert_eq!(task.status, TaskStatus::Pending);
    assert!(task.assigned_agent_id.is_none());
    assert!(task.dependencies.is_empty());
    assert_eq!(task.retry_policy.max_attempts, 1);
    assert_eq!(task.timeout.unwrap(), Duration::from_secs(300));
}

#[test]
fn test_task_builder_methods() {
    let agent_id = Uuid::new_v4();
    let dep_id = Uuid::new_v4();
    let policy = RetryPolicy { max_attempts: 3, backoff_ms: 500 };

    let task = Task::new("A", "B")
        .assign_agent(agent_id)
        .add_dependency(dep_id)
        .with_retry(policy)
        .with_timeout(Duration::from_secs(10));

    assert_eq!(task.assigned_agent_id, Some(agent_id));
    assert_eq!(task.dependencies.len(), 1);
    assert_eq!(task.dependencies[0], dep_id);
    assert_eq!(task.retry_policy.max_attempts, 3);
    assert_eq!(task.timeout.unwrap(), Duration::from_secs(10));
}

#[test]
fn test_agent_creation() {
    let agent = Agent::new("TestAgent", "Tester", "Test goal", "Test backstory");
    assert_eq!(agent.name, "TestAgent");
    assert_eq!(agent.role, "Tester");
    assert_eq!(agent.goal, "Test goal");
    assert_eq!(agent.backstory, "Test backstory");
    assert!(agent.tools.is_empty());
    assert!(agent.llm.is_none());
    assert!(matches!(agent.memory_scope, MemoryScope::Agent));
}

#[test]
fn test_agent_add_llm() {
    let llm: Arc<dyn LlmAdapter> = Arc::new(OpenAiAdapter::new("fake_key".to_string(), "gpt-4o".to_string()));
    let agent = Agent::new("A", "Role", "Goal", "Backstory").add_llm(llm);
    assert!(agent.llm.is_some());
}
