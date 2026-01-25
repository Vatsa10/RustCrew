mod core;
mod tools;
mod api;
mod memory;

use crate::core::agent::Agent;
use crate::core::task::Task;
use crate::core::crew::Crew;
use crate::core::scheduler::Scheduler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    // 1. Create Agents
    let researcher = Agent::new(
        "Researcher",
        "Researcher",
        "Find information about RustCrew features",
        "Expert at finding technical details"
    );
    let researcher_id = researcher.id;

    let writer = Agent::new(
        "Writer",
        "Technical Writer",
        "Summarize researcher findings",
        "Specializes in clear, concise docs"
    );
    let writer_id = writer.id;

    // 2. Create Tasks
    let task1 = Task::new(
        "Research CrewAI core concepts",
        "List of 5 core concepts from CrewAI"
    ).assign_agent(researcher_id);
    let task1_id = task1.id;

    let task2 = Task::new(
        "Write a summary of the findings",
        "A 3-paragraph summary of the researched concepts"
    ).assign_agent(writer_id).add_dependency(task1_id);

    // 3. Create Crew and Scheduler
    let mut crew = Crew::new(vec![researcher, writer]);
    crew.add_task(task1);
    crew.add_task(task2);

    let scheduler = Scheduler::new(crew);

    // 4. Run!
    println!("Starting RustCrew execution...");
    if let Err(e) = scheduler.run().await {
        eprintln!("Execution failed: {}", e);
    }
    println!("RustCrew execution finished.");
}
