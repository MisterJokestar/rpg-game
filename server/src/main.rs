//! Entry point for the RPG game server.
//!
//! Reads configuration from environment variables, connects to the selected
//! database backend (Firestore or MongoDB, chosen at compile time via Cargo
//! features), assembles [`AppState`], mounts the Axum router, and starts the
//! TCP listener.
//!
//! # Database backend selection
//!
//! Enable exactly one backend feature when building:
//!
//! ```bash
//! cargo run --features mongodb    # local development
//! cargo run --features firestore  # production / GCP
//! ```
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
use db::{UserRepository, GameRepository, CharacterRepository};
#[cfg(feature = "firestore")]
use db::firestore::FirestoreRepository;
#[cfg(feature = "mongodb")]
use db::mongodb::MongoRepository;
use state::AppState;
use tokio::sync::RwLock;

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

    #[cfg(feature = "firestore")]
    let (users, games, characters): (Arc<dyn UserRepository>, Arc<dyn GameRepository>, Arc<dyn CharacterRepository>) = {
        let project_id = config.firestore_project_id.as_deref()
            .expect("FIRESTORE_PROJECT_ID must be set");
        tracing::info!("Connecting to Firestore (project: {})", project_id);
        let repo = Arc::new(FirestoreRepository::new(project_id).await?);
        (repo.clone() as Arc<dyn UserRepository>, repo.clone() as Arc<dyn GameRepository>, repo as Arc<dyn CharacterRepository>)
    };

    #[cfg(feature = "mongodb")]
    let (users, games, characters): (Arc<dyn UserRepository>, Arc<dyn GameRepository>, Arc<dyn CharacterRepository>) = {
        tracing::info!("Connecting to MongoDB (uri: {})", config.mongodb_uri);
        let repo = Arc::new(MongoRepository::new(&config.mongodb_uri).await?);
        (repo.clone() as Arc<dyn UserRepository>, repo.clone() as Arc<dyn GameRepository>, repo as Arc<dyn CharacterRepository>)
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
