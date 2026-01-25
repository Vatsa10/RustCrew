# RustCrew

**RustCrew** is a Rust-native multi-agent orchestration framework designed for production-level autonomous agent workflows. It focuses on deterministic execution, safe concurrency via tokio, and low-latency task orchestration. The system implements the core primitives of agentic orchestration—specialized agents, task dependencies, and shared context—within a strongly-typed and performance-oriented runtime.

---

## Core Features

- **Asynchronous Service**: Built on top of `tokio` and `axum` as a managed service for remote agent orchestration with graceful shutdown support.
- **Robust Infrastructure**: Centralized configuration management and unified error handling for reliable production operations.
- **DAG-Based Task Scheduling**: Implements a directed acyclic graph (DAG) scheduler that resolves dependencies and parallelizes independent tasks.
- **Persistent Memory**: Deep integration with `sqlx` providing SQLite/Postgres persistence for long-running workflows and auditability.
- **RESTful Orchestration**: Standardized API for managing Crew specifications, tracking task state, and monitoring execution runs.
- **Extensible Tooling**: A robust `Tool` trait for building custom capabilities with built-in support for network and filesystem operations.

---

## Built-in Tools

RustCrew includes a standard library of tools to accelerate agent development:

- **HttpClient**: Asynchronous GET request tool using `reqwest` for web data retrieval.
- **FileLoader**: Secure local file access using `tokio::fs` for context loading.

---

## Getting Started

For a detailed walkthrough of how to install, configure, and run RustCrew, please see the **[Setup Guide](SETUP.md)**.

### Quick Start

1. **Configure**: Create a `.env` file with `SERVER_PORT` and `DATABASE_URL`.
2. **Run**: Start the server.
   ```bash
   cargo run
   ```

The server will listen on `0.0.0.0:3000` by default.

### API Integration Example

You can define a crew and initiate a run via the REST API. The following demonstrates creating a research-driven workflow:

**1. Create a Crew (POST /api/v1/crews)**

```json
{
  "name": "Market Analysis Crew",
  "agents": [
    {
      "name": "Researcher",
      "role": "Technical Analyst",
      "goal": "Identify 3 core trends in Rust ecosystem",
      "backstory": "Specialist in systems programming analysis."
    },
    {
      "name": "Writer",
      "role": "Technical Writer",
      "goal": "Summarize the identified trends",
      "backstory": "Expert at technical documentation."
    }
  ],
  "tasks": [
    {
      "description": "Research ecosystem trends",
      "expected_output": "List of 3 trends",
      "agent_index": 0
    },
    {
      "description": "Write summary",
      "expected_output": "Markdown summary report",
      "agent_index": 1
    }
  ]
}
```

**2. Trigger an Execution Run (POST /api/v1/crews/{id}/runs)**

Initiates the background scheduler to resolve the task graph.

3. **Interact**: The server listens on `0.0.0.0:3000` by default.
---

## System Architecture

RustCrew is engineered with modularity at its core:

- **API Layer**: Axum-based endpoints for managing resources and internal state, backed by unified error handling.
- **Execution Engine**: An asynchronous scheduler that manages the task lifecycle and worker synchronization.
- **Persistence Layer**: SQL-backed storage using `sqlx` (SQLite/Postgres) for task tracking and execution history.
- **Memory System**: Hybrid architecture supporting thread-safe in-memory caching and persistent SQL storage.
- **Agent Sandbox**: Encapsulates the logic, tools, and memory scope for specific agent roles.

---

## Roadmap

See [todo.md](.ai/todo.md) for the detailed project roadmap and current progress.

---

## Contributing

Technical contributions are welcome. Please ensure all modifications include relevant unit tests and follow established Rust idioms.

## License

This project is licensed under the Apache 2.0 License.
