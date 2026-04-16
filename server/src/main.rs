use std::sync::Arc;

mod config;
mod db;
mod error;
mod middleware;
mod models;
mod routes;
mod state;

use config::{Config, DatabaseBackend};
use db::{UserRepository, GameRepository};
use db::{firestore::FirestoreRepository, mongodb::MongoRepository};
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .init();

    let config = Config::from_env();

    let (users, games): (Arc<dyn UserRepository>, Arc<dyn GameRepository>) =
        match config.database_backend {
            DatabaseBackend::Firestore => {
                let project_id = config.firestore_project_id.as_deref()
                    .expect("FIRESTORE_PROJECT_ID must be set when DATABASE_BACKEND=firestore");
                tracing::info!("Connecting to Firestore (project: {})", project_id);
                let repo = Arc::new(FirestoreRepository::new(project_id).await?);
                (repo.clone(), repo)
            }
            DatabaseBackend::MongoDB => {
                let uri = config.mongodb_uri.as_deref().unwrap_or("mongodb://localhost:27017");
                tracing::info!("Connecting to MongoDB ({})", uri);
                let repo = Arc::new(MongoRepository::new(uri, config.mongodb_db_name.as_deref()).await?);
                (repo.clone(), repo)
            }
        };

    let state = Arc::new(AppState { users, games });
    let app = routes::create_router(state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Listening on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
