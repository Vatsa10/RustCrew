mod core;
mod tools;
mod api;
mod memory;

use std::sync::Arc;
use dashmap::DashMap;
use crate::api::handlers::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();
    
    let state = Arc::new(AppState {
        crews: DashMap::new(),
        runs: DashMap::new(),
    });

    let app = api::app(state);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("RustCrew API server listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}
