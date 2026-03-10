use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use crate::api::models::{CreateCrewRequest, CrewResponse, RunResponse};
use crate::core::crew::Crew;
use crate::core::scheduler::Scheduler;
use std::sync::Arc;
use tokio::sync::Mutex;
use dashmap::DashMap;

use crate::core::agent::Agent;
use crate::core::task::Task;
use crate::error::AppError;

pub struct AppState {
    pub crews: DashMap<Uuid, Arc<Mutex<Crew>>>,
    pub runs: DashMap<Uuid, String>,
}

pub async fn create_crew(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCrewRequest>,
) -> Result<(StatusCode, Json<CrewResponse>), AppError> {
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
    let crew_arc = Arc::new(Mutex::new(crew));
    state.crews.insert(crew_id, crew_arc);

    Ok((StatusCode::CREATED, Json(CrewResponse { id: crew_id, name: payload.name })))
}

pub async fn start_run(
    State(state): State<Arc<AppState>>,
    Path(crew_id): Path<Uuid>,
) -> Result<Json<RunResponse>, AppError> {
    let crew_arc = state.crews.get(&crew_id)
        .ok_or_else(|| AppError::NotFound(format!("Crew {} not found", crew_id)))?
        .clone();
    
    let run_id = Uuid::new_v4();
    state.runs.insert(run_id, "Running".to_string());
    
    let state_clone = state.clone();
    tokio::spawn(async move {
        let scheduler = Scheduler { crew: crew_arc };
        match scheduler.run().await {
            Ok(_) => {
                state_clone.runs.insert(run_id, "Completed".to_string());
            }
            Err(e) => {
                state_clone.runs.insert(run_id, format!("Failed: {}", e));
            }
        }
    });

    Ok(Json(RunResponse { id: run_id, status: "Started".to_string() }))
}

pub async fn get_run_status(
    State(state): State<Arc<AppState>>,
    Path(run_id): Path<Uuid>,
) -> Result<Json<RunResponse>, AppError> {
    let status = state.runs.get(&run_id).ok_or_else(|| AppError::NotFound(format!("Run {} not found", run_id)))?;
    Ok(Json(RunResponse { id: run_id, status: status.clone() }))
}
