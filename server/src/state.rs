//! Shared application state and per-game session handles.
//!
//! [`AppState`] is cloned into every route handler via Axum's `State`
//! extractor. [`GameSession`] holds the live channels and snapshot for a
//! single in-progress game.
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::{mpsc, RwLock, broadcast};
use crate::{
    db::{
        CharacterRepository,
        GameRepository,
        UserRepository
    },
    game::watcher::Watcher,
    models::{Action, game::{Game, SequencedEvent}},
};

/// Shared application state injected into every route handler via Axum's
/// `State` extractor. Wrap new shared resources (config, cache clients, etc.)
/// in this struct rather than reaching for globals.
pub struct AppState {
    /// Repository for user accounts.
    pub users: Arc<dyn UserRepository>,
    /// Repository for game records.
    pub games: Arc<dyn GameRepository>,
    /// Repository for player characters.
    pub characters: Arc<dyn CharacterRepository>,
    /// Map of currently active game sessions, keyed by game ID.
    pub sessions: RwLock<HashMap<String, GameSession>>,
}

/// Live handles for a single active game session.
///
/// Created by `start_game` and removed by `stop_game` or when the game ends
/// naturally.
pub struct GameSession {
    /// Sender half of the player-action channel. Clone this to submit actions.
    pub action_tx: mpsc::Sender<Action>,
    /// Broadcast sender for serialised game events. Subscribe to receive SSE
    /// updates.
    pub event_tx: broadcast::Sender<SequencedEvent>,
    /// Latest game snapshot together with its sequence number, shared between
    /// the runner and SSE streaming handlers.
    pub snapshot: Arc<RwLock<(u64, Game)>>,
    /// Watcher that monitors the session for inactivity and handles graceful
    /// shutdown.
    pub watcher: Watcher,
}
