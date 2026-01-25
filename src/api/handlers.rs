use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use crate::api::models::{CreateCrewRequest, CrewResponse, RunResponse};
use crate::core::crew::Crew;
use crate::core::agent::Agent;
use crate::core::task::Task;
use crate::core::scheduler::Scheduler;
use std::sync::Arc;
use dashmap::DashMap;

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
    let crew = state.crews.get(&crew_id).ok_or(StatusCode::NOT_FOUND)?.clone();
    
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
