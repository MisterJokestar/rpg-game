use std::sync::Arc;

mod config;
mod db;
mod error;
mod middleware;
mod models;
mod routes;
mod state;

use config::{Config, DatabaseBackend};
use db::{firestore::FirestoreRepository, mongodb::MongoRepository};
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load a .env file if one is present (no-op in production).
    dotenvy::dotenv().ok();

    // Structured logging — level controlled by RUST_LOG env var.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .init();

    let config = Config::from_env();

    // Select the database backend at startup based on DATABASE_BACKEND.
    let repository: Arc<dyn db::Repository> = match config.database_backend {
        DatabaseBackend::Firestore => {
            let project_id = config
                .firestore_project_id
                .as_deref()
                .expect("FIRESTORE_PROJECT_ID must be set when DATABASE_BACKEND=firestore");
            tracing::info!("Connecting to Firestore (project: {})", project_id);
            Arc::new(FirestoreRepository::new(project_id).await?)
        }
        DatabaseBackend::MongoDB => {
            let uri = config
                .mongodb_uri
                .as_deref()
                .unwrap_or("mongodb://localhost:27017");
            tracing::info!("Connecting to MongoDB ({})", uri);
            Arc::new(MongoRepository::new(uri, config.mongodb_db_name.as_deref()).await?)
        }
    };

    let state = Arc::new(AppState { db: repository });
    let app = routes::create_router(state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Listening on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
