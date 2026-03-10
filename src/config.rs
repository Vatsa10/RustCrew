use std::env;
use dotenvy::dotenv;
use directories::ProjectDirs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_port: String,
    pub database_url: String,
    pub storage_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
        
        let proj_dirs = ProjectDirs::from("com", "vatsa", "RustCrew")
            .expect("Could not determine project directories");
        let storage_path = proj_dirs.data_dir().to_path_buf();
        
        // Ensure directory exists
        std::fs::create_dir_all(&storage_path).ok();

        let default_db = format!("sqlite://{}/rustcrew.db?mode=rwc", storage_path.to_string_lossy());

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| default_db);

        Self {
            server_port,
            database_url,
            storage_path,
        }
    }
}
