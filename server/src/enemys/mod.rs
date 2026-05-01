//! Enemy definitions, the [`Enemy`] trait, and factory functions.
//!
//! Each enemy type lives in its own sub-module and implements the [`Enemy`]
//! trait. The [`EnemyType`] enum is persisted as part of the game state so
//! that the correct enemy implementation can be reconstructed when a session
//! is resumed.
use rand::random_range;
use serde::{Deserialize, Serialize};

use crate::{
    enemys::{
        bomb::BombEnemy, copper_sides::CopperSidesEnemy, dummy::DummyEnemy, iron_lotus::IronLotusEnemy, maestro::MaestroEnemy, rose_buddies::RoseBuddiesEnemy
    },
    models::{
        Action,
        game::{EnemyState, Game}
    }
};

pub mod dummy;
pub mod rose_buddies;
pub mod copper_sides;
pub mod maestro;
pub mod iron_lotus;
pub mod bomb;

/// Discriminant for every enemy type in the game.
///
/// Serialised alongside [`EnemyState`] so that the correct
/// [`Enemy`] implementation can be selected when reconstructing a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyType {
    /// Placeholder enemy that never acts.
    Dummy,
    /// Fast attacker with thorns and a heal cycle.
    RoseBuddies,
    /// Tanky enemy that alternates attack and defend, growing stronger each
    /// cycle.
    CopperSides,
    /// Ranged attacker that uses an ammo/reload mechanic.
    Maestro,
    /// Burns the player every turn with escalating damage.
    IronLotus,
    /// Counts down and detonates for massive damage.
    Bomb,
    /// Fallback for unknown variants encountered during deserialisation.
    #[serde(other)]
    Unknown,
}

/// Behaviour contract that all enemies must satisfy.
pub trait Enemy: Send + Sync {
    /// Return the initial [`EnemyState`] for a fresh encounter.
    fn get_new_state(&mut self) -> EnemyState;
    /// Return the number of turns until this enemy's next action, given the
    /// current game state.
    fn next_turn(&mut self, state: &mut Game) -> i64;
    /// Choose and return the action this enemy will take on its turn.
    fn choose_action(&mut self, state: &mut Game) -> Action;
    /// React to the player's action after the player's turn resolves.
    ///
    /// Used for abilities that trigger in response to the player (e.g.,
    /// thorns, counter-attacks).
    fn after_players_turn(&mut self, state: &mut Game, prev_action: &Action);
    /// Return a queued narrative message (if any) and clear it.
    ///
    /// Called after each turn to give the enemy a chance to announce an
    /// ability or state change to the player.
    fn message(&mut self) -> Option<String>;
}

/// Construct a boxed [`Enemy`] for the given [`EnemyType`].
///
/// Returns a [`DummyEnemy`] for [`EnemyType::Unknown`].
pub fn get_enemy_by_type(enemy_type: EnemyType) -> Box<dyn Enemy> {
    match enemy_type {
        EnemyType::Dummy => Box::new(DummyEnemy::new()),
        EnemyType::RoseBuddies => Box::new(RoseBuddiesEnemy::new()),
        EnemyType::CopperSides => Box::new(CopperSidesEnemy::new()),
        EnemyType::Maestro => Box::new(MaestroEnemy::new()),
        EnemyType::IronLotus => Box::new(IronLotusEnemy::new()),
        EnemyType::Bomb => Box::new(BombEnemy::new()),
        EnemyType::Unknown => Box::new(DummyEnemy::new()),
    }
}

/// Select a random non-Dummy enemy for the next round.
pub fn get_random_enemy() -> Box<dyn Enemy> {
    // range is exclusive 0..5 → 0 to 4 inclusive
    let rn = random_range(0..5);
    match rn {
        0 => Box::new(RoseBuddiesEnemy::new()),
        1 => Box::new(CopperSidesEnemy::new()),
        2 => Box::new(MaestroEnemy::new()),
        3 => Box::new(IronLotusEnemy::new()),
        4 => Box::new(BombEnemy::new()),
        _ => Box::new(DummyEnemy::new()),
    }
}
