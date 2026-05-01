//! Server configuration loaded from environment variables.
//!
//! [`Config::from_env`] reads each variable at startup, falling back to
//! sensible defaults when a variable is absent.
use std::env;

/// Runtime configuration for the server process.
///
/// Fields that are only meaningful for a particular database backend are
/// gated behind the corresponding Cargo feature flag.
#[derive(Debug, Clone)]
pub struct Config {
    /// Host address the TCP listener binds to. Defaults to `"0.0.0.0"`.
    pub host: String,
    /// TCP port the server listens on. Defaults to `5000`.
    pub port: u16,
    /// GCP project ID used to connect to Firestore.
    ///
    /// Required when the `firestore` feature is enabled; reads
    /// `FIRESTORE_PROJECT_ID` from the environment.
    #[cfg(feature = "firestore")]
    pub firestore_project_id: Option<String>,
    /// MongoDB connection URI.
    ///
    /// Reads `MONGODB_URI` from the environment; defaults to
    /// `"mongodb://localhost:27017"` for local development.
    #[cfg(feature = "mongodb")]
    pub mongodb_uri: String,
}

impl Config {
    /// Build a [`Config`] by reading environment variables.
    ///
    /// Missing variables fall back to the defaults documented on each field.
    /// An invalid `PORT` value (non-numeric) silently falls back to `5000`.
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
