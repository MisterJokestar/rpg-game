use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::{mpsc, RwLock, broadcast};
use uuid::Uuid;
use crate::{
    db::{GameRepository, UserRepository}, 
    game::watcher::Watcher, 
    models::{Action, game::GameEvent},
};

/// Shared application state injected into every route handler via axum's
/// `State` extractor. Wrap new shared resources (config, cache clients, etc.)
/// in this struct rather than reaching for globals.
pub struct AppState {
    pub users: Arc<dyn UserRepository>,
    pub games: Arc<dyn GameRepository>,
    pub sessions: RwLock<HashMap<Uuid, GameSession>>,
}

pub struct GameSession {
    pub action_tx: mpsc::Sender<Action>,
    pub event_tx: broadcast::Sender<GameEvent>,
    pub watcher: Watcher,
}
