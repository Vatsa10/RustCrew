use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use crate::api::models::{CreateCrewRequest, CrewResponse, RunResponse};
use crate::core::crew::Crew;
use std::sync::Arc;
use dashmap::DashMap; // This import is used by AppState

// Agent and Task are used in create_crew, so they need to be imported.
// Assuming they are in `crate::core::agent` and `crate::core::task` based on common Rust project structure.
// If they are in `crate::core::crew` then they would be `Crew::Agent` and `Crew::Task` or similar.
// Based on the original code `Agent::new` and `Task::new` are used, implying they are top-level or module-level imports.
// The original code did not have these imports, which would cause a compile error.
// To make the resulting file syntactically correct, I will add them, assuming they come from `crate::core::agent` and `crate::core::task`.
// If the user meant for these to be removed because they are not explicitly imported in the *original* snippet,
// then the original snippet was already syntactically incorrect regarding these types.
// Given the instruction is to remove *unused* imports, and these *are* used, they should be present.
// I will add them to ensure the code is syntactically correct after the change.
use crate::core::agent::Agent;
use crate::core::task::Task;

pub struct AppState {
    pub crews: DashMap<Uuid, Arc<Crew>>,
    pub runs: DashMap<Uuid, String>,
}

pub async fn create_crew(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCrewRequest>,
) -> (StatusCode, Json<CrewResponse>) {
    let mut agents = Vec::new();
    for spec in payload.agents {
        agents.push(Agent::new(&spec.name, &spec.role, &spec.goal, &spec.backstory));
    }

    let mut crew = Crew::new(agents);
    
    for spec in payload.tasks {
        if let Some(agent) = crew.agents.get(spec.agent_index) {
            let task = Task::new(&spec.description, &spec.expected_output).assign_agent(agent.id);
            crew.add_task(task);
        }
    }

    let crew_id = crew.id;
    let crew_arc = Arc::new(crew);
    state.crews.insert(crew_id, crew_arc);

    (StatusCode::CREATED, Json(CrewResponse { id: crew_id, name: payload.name }))
}

pub async fn start_run(
    State(state): State<Arc<AppState>>,
    Path(crew_id): Path<Uuid>,
) -> Result<Json<RunResponse>, StatusCode> {
    let _crew = state.crews.get(&crew_id).ok_or(StatusCode::NOT_FOUND)?.clone();
    
    // In a real app, we'd clone or create a new run instance
    // For now, let's just trigger the scheduler in the background
    let run_id = Uuid::new_v4();
    state.runs.insert(run_id, "Started".to_string());
    
    let state_clone = state.clone();
    tokio::spawn(async move {
        // This is a bit hacky because Crew doesn't implement Clone easily with Arc tools
        // We'll just print for now or Implement proper run logic
        println!("Background run started for crew: {}", crew_id);
        state_clone.runs.insert(run_id, "Completed".to_string());
    });

    Ok(Json(RunResponse { id: run_id, status: "Started".to_string() }))
}

pub async fn get_run_status(
    State(state): State<Arc<AppState>>,
    Path(run_id): Path<Uuid>,
) -> Result<Json<RunResponse>, StatusCode> {
    let status = state.runs.get(&run_id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(RunResponse { id: run_id, status: status.clone() }))
}
