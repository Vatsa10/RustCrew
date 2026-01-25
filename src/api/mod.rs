pub mod handlers;
pub mod models;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::api::handlers::{create_crew, start_run, get_run_status, AppState};

pub fn app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/crews", post(create_crew))
        .route("/api/v1/crews/:id/runs", post(start_run))
        .route("/api/v1/runs/:id", get(get_run_status))
        .with_state(state)
}
