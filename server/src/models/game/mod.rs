//! Game state model.
//!
//! Defines the persistent [`Game`] record, the per-combatant state types
//! ([`PlayerState`], [`EnemyState`], [`Health`]), the [`Combatant`] trait
//! that drives combat calculations, and the event types broadcast to SSE
//! clients during a live session.
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::game::{
    combatant::Combatant,
    status::StatusEffects
};

pub mod status;
pub mod combatant;
pub mod leaderboard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// Unique game identifier (UUID v7).
    #[cfg_attr(feature = "firestore", serde(rename = "_firestore_id"))]
    #[cfg_attr(feature = "mongodb", serde(rename = "_id"))]
    pub id: String,
    /// `Some(true)` if the player won, `Some(false)` if they lost, `None`
    /// while the game is still in progress.
    pub win: Option<bool>,
    /// Current round number (incremented each time an enemy is defeated).
    pub round: i64,
    /// Monotonically-increasing turn counter used by the turn-order system.
    pub turn: i64,
    /// Holds the various states of the combatants.
    pub combatants: HashMap<String, Combatant>,
    /// Reference for the current turn.
    pub current_combatant: Option<String>,
    /// Running tally of how many of each enemy type the player has defeated.
    #[serde(default)]
    pub enemies_defeated: HashMap<String, i64>,
    pub status_effects: StatusEffects
}

/// API response shape for a game — always serialises the ID as `"id"`
/// regardless of the active database backend.
#[derive(Debug, Clone, Serialize)]
pub struct GameResponse {
    /// Unique game identifier.
    pub id: String,
    /// Whether the game has ended.
    pub complete: bool,
    /// `Some(true)` = player won, `Some(false)` = player lost, `None` = in progress.
    pub win: Option<bool>,
    /// Current round number.
    pub round: i64,
    /// Monotonically-increasing turn counter.
    pub turn: i64,
    /// Holds the various states of the combatants.
    pub combatants: HashMap<String, Combatant>,
    /// Reference for the current turn.
    pub current_combatant: Option<String>,
    /// Running tally of enemies defeated, keyed by enemy type.
    pub enemies_defeated: HashMap<String, i64>,
}

impl From<Game> for GameResponse {
    fn from(g: Game) -> Self {
        GameResponse {
            id: g.id,
            complete: g.complete,
            win: g.win,
            round: g.round,
            turn: g.turn,
            combatants: g.combatants,
            current_combatant: g.current_combatant,
            enemies_defeated: g.enemies_defeated,
        }
    }
}

/// An event broadcast to SSE subscribers during a live game session.
#[derive(Debug, Clone, Serialize)]
pub enum GameEvent {
    /// A turn was resolved; contains the updated game state.
    TurnResolved(GameResponse),
    /// The game ended naturally (player died or won); contains the final state.
    GameOver(GameResponse),
    /// The game was stopped externally (e.g., client called `stop_game`);
    /// contains the state at time of stopping.
    GameStopped(GameResponse),
    /// A narrative or status message from the game engine (e.g., enemy
    /// ability announcements).
    GameMessage(String),
}

/// A [`GameEvent`] tagged with a monotonically-increasing sequence number.
///
/// SSE clients use the sequence number to detect and discard events that are
/// older than the initial snapshot they received on connection.
#[derive(Debug, Clone, Serialize)]
pub struct SequencedEvent {
    /// Sequence number for this event. Starts at 1 and increments with every
    /// event emitted during a session.
    pub seq: u64,
    /// The game event payload.
    pub event: GameEvent,
}

/// Request body for the `POST /game/new` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateGameRequest {
    /// ID of the player starting the game.
    pub player_id: String,
    /// ID of the character the player wants to use.
    pub character_id: String,
}

/// Response body returned after a successful `POST /game/new`.
#[derive(Debug, Clone, Serialize)]
pub struct CreateGameResponse {
    /// ID of the newly created game.
    pub game_id: String,
}

impl Game {
    /// Create a new game for the given player and character with starting HP
    /// of 100, a randomly selected first enemy, and all counters zeroed.
    pub fn new(player: String, character: String) -> Self {
        Game {
            id: Uuid::now_v7().to_string(),
            win: None,
            round: 0,
            turn: 0,
            combatants: HashMap::new(),
            current_combatant: None,
            enemies_defeated: HashMap::new(),
            status_effects: StatusEffects::new(),
        }
    }
}

