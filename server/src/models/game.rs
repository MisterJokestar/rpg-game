//! Game state model.
//!
//! Defines the persistent [`Game`] record, the per-combatant state types
//! ([`PlayerState`], [`EnemyState`], [`Health`]), the [`Combatant`] trait
//! that drives combat calculations, and the event types broadcast to SSE
//! clients during a live session.
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enemys::{EnemyType, get_random_enemy};

/// A game record stored in the database.
///
/// Tracks the full combat state across multiple rounds. A new enemy is
/// spawned for each round; the game ends when the player reaches 0 HP or
/// after completing a sufficient number of rounds (see
/// [`crate::game::runner::Runner`]).
///
/// The `id` field is serialised as `_firestore_id` or `_id` depending on the
/// active database backend feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// Unique game identifier (UUID v7).
    #[cfg_attr(feature = "firestore", serde(rename = "_firestore_id"))]
    #[cfg_attr(feature = "mongodb", serde(rename = "_id"))]
    pub id: String,
    /// Whether the game has ended (player died or won).
    pub complete: bool,
    /// `Some(true)` if the player won, `Some(false)` if they lost, `None`
    /// while the game is still in progress.
    pub win: Option<bool>,
    /// Current round number (incremented each time an enemy is defeated).
    pub round: i64,
    /// Monotonically-increasing turn counter used by the turn-order system.
    pub turn: i64,
    /// Snapshot of the player's in-game state.
    pub player_state: PlayerState,
    /// Snapshot of the current enemy's in-game state.
    pub enemy_state: EnemyState,
    /// Running tally of how many of each enemy type the player has defeated.
    #[serde(default)]
    pub enemies_defeated: HashMap<EnemyType, i64>,
}

/// Live combat state for the player.
///
/// Includes cumulative statistics used to build a post-game summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    /// ID of the owning user.
    pub player_id: String,
    /// ID of the character being used in this game.
    pub character_id: String,
    /// The turn value on which the player will next act, or `None` if not yet
    /// calculated for this round.
    pub next_turn: Option<i64>,
    /// Current and maximum HP.
    pub health: Health,
    /// Damage the player will absorb before taking HP loss this turn.
    pub block: i64,
    /// Cumulative HP lost over the entire game.
    pub damage_taken: i64,
    /// Cumulative HP restored over the entire game.
    pub damage_healed: i64,
    /// Cumulative damage prevented by blocking over the entire game.
    pub damage_blocked: i64,
    /// Cumulative damage avoided through dodge mechanics over the entire game.
    pub damage_dodged: i64,
    /// Cumulative damage dealt to enemies over the entire game.
    pub damage_dealt: i64,
}

/// Live combat state for the current enemy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyState {
    /// The turn value on which the enemy will next act, or `None` if not yet
    /// calculated.
    pub next_turn: Option<i64>,
    /// Enemy-specific internal state counter used to implement multi-turn
    /// behaviour patterns (e.g., attack/defend cycles).
    pub state: i64,
    /// Current and maximum HP.
    pub health: Health,
    /// Damage the enemy will absorb before taking HP loss this turn.
    pub block: i64,
    /// Which enemy type this state belongs to.
    pub enemy_type: EnemyType,
}

/// Current and maximum hit points for a combatant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    /// Current HP. Clamped to `[0, max]` by the combat engine.
    pub current: i64,
    /// Maximum HP.
    pub max: i64,
}

/// Behaviour that both the player and enemies must implement for the combat
/// engine to resolve a turn.
pub trait Combatant {
    /// Set the combatant's block value for this turn.
    fn set_block(&mut self, block: i64);
    /// Clear the combatant's block value (called at the start of each turn).
    fn reset_block(&mut self);
    /// Apply `damage` to the combatant after subtracting block.
    ///
    /// Returns the actual HP lost (after block reduction), capped so that
    /// excess damage beyond 0 HP is not counted.
    fn take_damage(&mut self, damage: i64) -> i64;
    /// Record `damage` as having been dealt (used for player statistics).
    fn deal_damage(&mut self, damage: i64);
    /// Restore `healing` HP, capped at maximum HP.
    fn heal_damage(&mut self, healing: i64);
}

impl Combatant for PlayerState {
    fn set_block(&mut self, block: i64) {
        self.block = block;
    }

    fn reset_block(&mut self) {
        self.block = 0;
    }

    fn take_damage(&mut self, damage: i64) -> i64 {
        // get damage after block
        let attack_dmg = damage - self.block;
        // there is unblocked damage.
        if attack_dmg > 0 {
            self.damage_blocked += self.block;
            self.block = 0;
            let mut current_hp = self.health.current;
            // HP after unblocked damage applied.
            current_hp -= attack_dmg;
            let damage_taken;
            // If HP is reduced to 0, excess damage not logged
            if current_hp <= 0 {
                self.health.current = 0;
                damage_taken = attack_dmg + current_hp;
            } else {
                self.health.current = current_hp;
                damage_taken = attack_dmg;
            }
            // Log damage taken.
            self.damage_taken += damage_taken;
            // returns damage taken.
            damage_taken
        } else {
            // All damage was blocked.
            self.damage_blocked = damage + attack_dmg;
            // remaining block is kept.
            self.block = -attack_dmg;
            0
        }
    }

    fn deal_damage(&mut self, damage: i64) {
        // just for logging purposes
        self.damage_dealt += damage;
    }

    fn heal_damage(&mut self, healing: i64) {
        // Healing shouldn't overflow, caps at total_hp
        if healing + self.health.current > self.health.max {
            self.damage_healed += self.health.max - self.health.current;
            self.health.current = self.health.max;
        } else {
            self.damage_healed += healing;
            self.health.current += healing;
        }
    }
}

// Same as impl for PlayerState, but without logging.
impl Combatant for EnemyState {
    fn set_block(&mut self, block: i64) {
        self.block = block;
    }

    fn reset_block(&mut self) {
        self.block = 0;
    }

    fn take_damage(&mut self, damage: i64) -> i64 {
        let attack_dmg = damage - self.block;
        if attack_dmg > 0 {
            self.block = 0;
            let mut current_hp = self.health.current;
            current_hp -= attack_dmg;
            let damage_taken;
            if current_hp <= 0 {
                self.health.current = 0;
                damage_taken = attack_dmg + current_hp;
            } else {
                self.health.current = current_hp;
                damage_taken = attack_dmg;
            }
            damage_taken
        } else {
            self.block = -attack_dmg;
            0
        }
    }

    fn deal_damage(&mut self, damage: i64) {
        _ = damage;
    }

    fn heal_damage(&mut self, healing: i64) {
        if healing + self.health.current > self.health.max {
            self.health.current = self.health.max;
        } else {
            self.health.current += healing;
        }
    }
}

/// An event broadcast to SSE subscribers during a live game session.
#[derive(Debug, Clone, Serialize)]
pub enum GameEvent {
    /// A turn was resolved; contains the updated game state.
    TurnResolved(Game),
    /// The game ended naturally (player died or won); contains the final state.
    GameOver(Game),
    /// The game was stopped externally (e.g., client called `stop_game`);
    /// contains the state at time of stopping.
    GameStopped(Game),
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

#[derive(Debug, Clone, Serialize)]
pub struct LeaderboardEntry {
    pub player_name: String,
    pub wins: i64,
    pub rounds: i64,
    pub damage_dealt: i64,
    pub enemies_defeated: i64,
}

/// Request body for the `POST /game/new` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateGameRequest {
    /// ID of the player starting the game.
    pub player_id: String,
    /// ID of the character the player wants to use.
    pub character_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateGameResponse {
    pub game_id: String,
}

impl Game {
    /// Create a new game for the given player and character with starting HP
    /// of 100, a randomly selected first enemy, and all counters zeroed.
    pub fn new(player: String, character: String) -> Self {
        Game {
            id: Uuid::now_v7().to_string(),
            complete: false,
            win: None,
            round: 0,
            turn: 0,
            player_state: PlayerState {
    			player_id: player,
    			character_id: character,
    			next_turn: None,
    			health: Health {
                    current: 100,
                    max: 100
                },
    			block: 0,
    			damage_taken: 0,
    			damage_healed: 0,
    			damage_blocked: 0,
    			damage_dodged: 0,
    			damage_dealt: 0,
            },
            enemy_state: get_random_enemy().get_new_state(),
            enemies_defeated: HashMap::new(),
        }
    }

    /// Reset player HP and spawn a fresh random enemy, preparing the game
    /// record to be used in a new session.
    pub fn set_up(&mut self) {
        self.player_state.next_turn = None;
        self.player_state.health = Health {
            current: 100,
            max: 100
        };
        self.enemy_state = get_random_enemy().get_new_state();
    }
}

impl LeaderboardEntry {
    pub fn new(player_name: String) -> Self {
        LeaderboardEntry {
            player_name,
            wins: 0,
            rounds: 0,
            damage_dealt: 0,
            enemies_defeated: 0
        }
    }
}
