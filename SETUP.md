# RustCrew Setup Guide

This guide will help you set up the RustCrew project locally. It covers prerequisites, installation, configuration, and running your first agent workflow.

## 1. Prerequisites

Before you begin, ensure you have the following installed:

- **Rust & Cargo**: Version 1.75+. Install via [rustup.rs](https://rustup.rs).
- **SQLite**: (Optional) CLI tool for inspecting the database. The `sqlx` crate handles the runtime connection.
- **Git**: For cloning the repository.

## 2. Installation

Clone the repository and install dependencies:

```bash
git clone https://github.com/Vatsa10/RustCrew.git
cd rustcrew
cargo build
```

## 3. Configuration

RustCrew uses a `.env` file for configuration.

1.  Create a `.env` file in the project root:

    ```bash
    cp .env.example .env  # If example exists, otherwise create new
    ```

2.  Add the following variables:

    ```env
    # Server Configuration
    SERVER_PORT=3000
    RUST_LOG=info

    # Database Configuration
    # Uses a local SQLite file named 'nova.db'
    DATABASE_URL=sqlite://nova.db?mode=rwc
    ```

## 4. Running the Server

Start the application:

```bash
cargo run
```

You should see:
```text
RustCrew API server listening on 0.0.0.0:3000
```

## 5. Usage Example

You can interact with the API using `curl` or Postman.

### Create a Crew

```bash
curl -X POST http://localhost:3000/api/v1/crews \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Demo Crew",
    "agents": [
      {
        "name": "Researcher",
        "role": "Analyst",
        "goal": "Find Rust facts",
        "backstory": "A curious bot"
      }
    ],
    "tasks": [
      {
        "description": "Find 2 cool Rust stats",
        "expected_output": "Stats list",
        "agent_index": 0
      }
    ]
  }'
```

---

## Troubleshooting

- **Database Errors**: Ensure `DATABASE_URL` is set correctly. If using SQLite, ensure the user has write permissions to the directory.
- **Missing Dependencies**: Run `cargo fetch` to re-download crates.
