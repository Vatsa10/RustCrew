# RustCrew

**RustCrew** is a Rust-native multi-agent orchestration framework designed for production-level autonomous agent workflows. It focuses on deterministic execution, safe concurrency via tokio, and low-latency task orchestration. The system implements the core primitives of agentic orchestration—specialized agents, task dependencies, and shared context—within a strongly-typed and performance-oriented runtime.

---

## Core Features

- **Asynchronous Runtime**: Built on top of `tokio` for efficient resource management and high-concurrency execution.
- **DAG-Based Task Scheduling**: Implements a directed acyclic graph (DAG) scheduler that resolves dependencies and parallelizes independent tasks.
- **Role-Based Agent Model**: Define agents with discrete roles, operational goals, and backstories for specialized execution.
- **Tooling Interface**: A standard `Tool` trait that enables external capabilities such as HTTP requests, browser automation, and data retrieval.
- **Concurrent Memory Access**: Thread-safe in-memory state management using concurrent data structures for agent and crew context.
- **Deterministic Replay Ready**: Designed to support recorded execution traces for auditing and debugging.

---

## Technical Quick Start

### Dependency Configuration

Include `rustcrew` in your `Cargo.toml`:

```toml
[dependencies]
rustcrew = { git = "https://github.com/Vatsa10/RustCrew" }
```

### Implementation Example

The following example demonstrates a multi-agent workflow where a writer task is dependent on the completion of a research task.

```rust
use rustcrew::core::agent::Agent;
use rustcrew::core::task::Task;
use rustcrew::core::crew::Crew;
use rustcrew::core::scheduler::Scheduler;

#[tokio::main]
async fn main() {
    // 1. Initialize specialised agents
    let researcher = Agent::new(
        "Researcher",
        "Technical Analyst",
        "Investigate Rust async memory safety patterns",
        "Expert in systems programming and memory management."
    );
    let researcher_id = researcher.id;

    let writer = Agent::new(
        "Writer",
        "Documentation Engineer",
        "Compile technical findings into a concise summary",
        "Specializes in technical communication and precision."
    );
    let writer_id = writer.id;

    // 2. Define Tasks and dependency graph
    let research_task = Task::new(
        "Analyze memory safety in async contexts",
        "A structured report on safety patterns"
    ).assign_agent(researcher_id);
    let research_task_id = research_task.id;

    let write_task = Task::new(
        "Summarize analysis",
        "A technical summary document"
    ).assign_agent(writer_id)
     .add_dependency(research_task_id);

    // 3. Orchestrate Crew execution
    let mut crew = Crew::new(vec![researcher, writer]);
    crew.add_task(research_task);
    crew.add_task(write_task);

    let scheduler = Scheduler::new(crew);
    if let Err(e) = scheduler.run().await {
        eprintln!("Workflow execution error: {}", e);
    }
}
```

---

## System Architecture

RustCrew is engineered with modularity at its core:

- **Agent Engine**: Encapsulates the logic, tools, and memory scope for a specific role.
- **Task Management**: Manages state transitions (Pending, Running, Completed, Failed) and output handling.
- **Orchestration Runtime**: A scheduler that manages the task lifecycle and handles async worker synchronization.
- **Tool Router**: A unified interface for agents to interact with system-level or external services.

---

## Roadmap

- **Inference Adapters**: Integration with OpenAI, Anthropic, and local LLM backends (via candle).
- **Tool Ecosystem**: Standard library of tools for web search, file I/O, and database interaction.
- **Observability Layer**: Tracing and metrics export via OpenTelemetry.
- **Persistence Layer**: SQL backends (SQLite/Postgres) for long-term agent memory.
- **Sandboxing**: WASM-based tool execution for secure, isolated operations.

---

## Contributing

Technical contributions are welcome. Please ensure all modifications include relevant unit tests and follow established Rust idioms.

## License

This project is licensed under the Apache 2.0 License.
