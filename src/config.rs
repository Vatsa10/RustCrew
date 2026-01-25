use std::env;
use dotenvy::dotenv;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_port: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
        
        // Default to a file-based DB if not set
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://nova.db?mode=rwc".to_string());

        Self {
            server_port,
            database_url,
        }
    }
}
