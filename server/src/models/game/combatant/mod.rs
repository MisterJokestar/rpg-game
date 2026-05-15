use serde::{Deserialize, Serialize};

use crate::models::game::combatant::{
    enemy::{EnemyType}, hero::Hero
};

pub mod action;
pub mod enemy;
pub mod hero;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combatant {
    health: Health,
    state: CombatantState,
    next_turn: Option<i64>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatantType {
    Hero(Hero),
    Enemy(EnemyType)
}

/// Live combat state for the player.
///
/// Includes cumulative statistics used to build a post-game summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatantState {
    /// Cumulative HP lost over the entire game. (Or round for an enemy)
    pub damage_taken: i64,
    /// Cumulative HP restored over the entire game. (Or round for an enemy)
    pub damage_healed: i64,
    /// Cumulative damage prevented by blocking over the entire game.
    /// (Or round for an enemy)
    pub damage_blocked: i64,
    /// Cumulative damage avoided through dodge mechanics over the entire game.
    /// (Or round for an enemy)
    pub damage_dodged: i64,
    /// Cumulative damage dealt to enemies over the entire game.
    /// (Or round for an enemy)
    pub damage_dealt: i64,
}

/// Current and maximum hit points for a combatant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    /// Current HP. Clamped to `[0, max]` by the combat engine.
    pub current: i64,
    /// Maximum HP.
    pub max: i64,
    /// Damage the player will absorb before taking HP loss this turn.
    pub block: i64,
}

impl Health {
    pub fn set(max: i64) -> Self {
        Health {
            current: max,
            max,
            block: 0
        }
    }

    pub fn heal(&mut self, health: i64) {
        self.current += health;
        if self.current > self.max { self.current = self.max; }
    }

    pub fn hurt(&mut self, damage: i64) {
        if self.block >= damage {
            self.block -= damage;
        } else {
            self.block = 0;
            self.current -= damage - self.block;
            if self.current == 0 { self.current = 0; }
        }
    }

    pub fn defend(&mut self, defense: i64) {
        self.block = defense;
    }
}
