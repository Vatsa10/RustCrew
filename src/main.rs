mod core;
mod tools;
mod api;
mod memory;
mod config;
mod error;

use std::sync::Arc;
use dashmap::DashMap;
use crate::api::handlers::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use tokio::signal;

#[tokio::main]
async fn main() {
    // 1. Load configuration
    let config = Config::new();

    // 2. Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();
    
    let state = Arc::new(AppState {
        crews: DashMap::new(),
        runs: DashMap::new(),
    });

    let app = api::app(state);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.server_port)).await.unwrap();
    println!("RustCrew API server listening on 0.0.0.0:{}", config.server_port);
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Signal received, starting graceful shutdown...");
}
