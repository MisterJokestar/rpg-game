use std::sync::Arc;
use crate::db::{UserRepository, GameRepository};

/// Shared application state injected into every route handler via axum's
/// `State` extractor. Wrap new shared resources (config, cache clients, etc.)
/// in this struct rather than reaching for globals.
pub struct AppState {
    pub users: Arc<dyn UserRepository>,
    pub games: Arc<dyn GameRepository>,
}
