use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    // Firestore
    pub firestore_project_id: Option<String>,
    // Server
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            firestore_project_id: env::var("FIRESTORE_PROJECT_ID").ok(),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .unwrap_or(5000),
        }
    }
}
