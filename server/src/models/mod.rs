//! Domain model types shared across the application.
//!
//! This module re-exports the three model sub-modules and defines [`Action`],
//! the enum that represents a single combat action taken by a player or enemy
//! during their turn.
use serde::Deserialize;

pub mod user;
pub mod character;
pub mod game;

/// A combat action that can be taken during a turn.
///
/// The `i64` payload in each variant is a *stat value* (power, defense, etc.)
/// that the game engine scales into an actual damage/block/heal amount using a
/// random multiplier.
#[derive(Debug, Clone, Deserialize)]
pub enum Action {
    /// Deal damage to the opponent scaled from the attacker's power stat.
    Attack(i64),
    /// Set a block value scaled from the defender's defense stat, absorbing
    /// incoming damage until the block is consumed or the turn ends.
    Defend(i64),
    /// Restore HP to the acting combatant scaled from their defense stat.
    Heal(i64),
    /// Pass the turn without taking any action.
    None,
}
