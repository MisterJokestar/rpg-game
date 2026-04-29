use std::collections::HashMap;
use std::sync::Arc;

mod config;
mod db;
mod error;
mod middleware;
mod models;
mod routes;
mod state;
mod enemys;
mod game;

use config::Config;
use db::{UserRepository, GameRepository};
use db::firestore::FirestoreRepository;
use state::AppState;
use tokio::sync::RwLock;

use crate::db::CharacterRepository;
use crate::state::GameSession;

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

    let (users, games, characters): (Arc<dyn UserRepository>, Arc<dyn GameRepository>, Arc<dyn CharacterRepository>) =
        {
            let project_id = config.firestore_project_id.as_deref()
                .expect("FIRESTORE_PROJECT_ID must be set when DATABASE_BACKEND=firestore");
            tracing::info!("Connecting to Firestore (project: {})", project_id);
            let repo = Arc::new(FirestoreRepository::new(project_id).await?);
            (repo.clone(), repo.clone(), repo)
        };

    let sessions: RwLock<HashMap<String, GameSession>> = RwLock::new(HashMap::new());

    let state = Arc::new(AppState { users, games, characters, sessions });
    let app = routes::create_router(state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Listening on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
