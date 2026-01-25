# RustCrew

**RustCrew** is a Rust-native multi-agent orchestration framework designed for production-level autonomous agent workflows. It focuses on deterministic execution, safe concurrency via tokio, and low-latency task orchestration. The system implements the core primitives of agentic orchestration—specialized agents, task dependencies, and shared context—within a strongly-typed and performance-oriented runtime.

---

## Core Features

- **Asynchronous Service**: Built on top of `tokio` and `axum` as a managed service for remote agent orchestration.
- **DAG-Based Task Scheduling**: Implements a directed acyclic graph (DAG) scheduler that resolves dependencies and parallelizes independent tasks.
- **Persistent Memory**: Deep integration with `sqlx` providing SQLite/Postgres persistence for long-running workflows and auditability.
- **RESTful Orchestration**: Standardized API for managing Crew specifications and monitoring background execution runs.
- **Role-Based Agent Model**: Define agents with discrete roles, operational goals, and backstories for specialized execution.
- **Tooling Interface**: A standard `Tool` trait that enables external capabilities such as HTTP requests, browser automation, and data retrieval.

---

## Technical Start

### Running the Orchestrator

Start the RustCrew server:

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

---

## System Architecture

RustCrew is engineered with modularity at its core:

- **API Layer**: Axum-based endpoints for managing resources and internal state.
- **Execution Engine**: An asynchronous scheduler that manages the task lifecycle and worker synchronization.
- **Persistence Layer**: SQL-backed storage using `sqlx` for task tracking and execution history.
- **Agent Sandbox**: Encapsulates the logic, tools, and memory scope for specific agent roles.

---

## Roadmap

- **Inference Adapters**: Integration with OpenAI, Anthropic, and local LLM backends (via candle).
- **Advanced Observability**: Distributed tracing and metrics export via OpenTelemetry.
- **Streamed Execution**: WebSocket/Server-Sent Events for real-time progress monitoring.
- **Sandboxing**: WASM-based tool execution for secure, isolated operations.

---

## Contributing

Technical contributions are welcome. Please ensure all modifications include relevant unit tests and follow established Rust idioms.

## License

This project is licensed under the Apache 2.0 License.
