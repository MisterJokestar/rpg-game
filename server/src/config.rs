use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    #[cfg(feature = "firestore")]
    pub firestore_project_id: Option<String>,
    #[cfg(feature = "mongodb")]
    pub mongodb_uri: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .unwrap_or(5000),
            #[cfg(feature = "firestore")]
            firestore_project_id: env::var("FIRESTORE_PROJECT_ID").ok(),
            #[cfg(feature = "mongodb")]
            mongodb_uri: env::var("MONGODB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
        }
    }
}
