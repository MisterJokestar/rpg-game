use std::env;

#[derive(Debug, Clone)]
pub enum DatabaseBackend {
    Firestore,
    MongoDB,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_backend: DatabaseBackend,

    // Firestore
    pub firestore_project_id: Option<String>,

    // MongoDB
    pub mongodb_uri: Option<String>,
    pub mongodb_db_name: Option<String>,

    // Server
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let backend = match env::var("DATABASE_BACKEND")
            .unwrap_or_else(|_| "mongodb".to_string())
            .to_lowercase()
            .as_str()
        {
            "firestore" => DatabaseBackend::Firestore,
            _ => DatabaseBackend::MongoDB,
        };

        Config {
            database_backend: backend,
            firestore_project_id: env::var("FIRESTORE_PROJECT_ID").ok(),
            mongodb_uri: env::var("MONGODB_URI").ok(),
            mongodb_db_name: env::var("MONGODB_DB_NAME").ok(),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
        }
    }
}
