//! Enemy definitions, the [`Enemy`] trait, and factory functions.
//!
//! Each enemy type lives in its own sub-module and implements the [`Enemy`]
//! trait. The [`EnemyType`] enum is persisted as part of the game state so
//! that the correct enemy implementation can be reconstructed when a session
//! is resumed.
use rand::random_range;
use serde::{Deserialize, Serialize};

use crate::models::game::{
    Game,
    combatant::{
        action::ActionCard,
        enemy::{
            rose_buddies::RoseBuddies,
            copper_sides::CopperSides,
            maestro::Maestro,
            iron_lotus::IronLotus,
            bomb::Bomb,
        },
    },
};

pub mod rose_buddies;
pub mod copper_sides;
pub mod maestro;
pub mod iron_lotus;
pub mod bomb;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnemyType {
    /// Placeholder enemy that never acts.
    Dummy,
    /// Fast attacker with thorns and a heal cycle.
    RoseBuddies(RoseBuddies),
    /// Tanky enemy that alternates attack and defend, growing stronger each
    /// cycle.
    CopperSides(CopperSides),
    /// Ranged attacker that uses an ammo/reload mechanic.
    Maestro(Maestro),
    /// Burns the player every turn with escalating damage.
    IronLotus(IronLotus),
    /// Counts down and detonates for massive damage.
    Bomb(Bomb),
}

impl EnemyType {
    /// Select a random non-Dummy enemy for the next round.
    pub fn random_enemy(reference:String) -> EnemyType {
        // range is exclusive 0..5 → 0 to 4 inclusive
        let rn = random_range(0..5);
        match rn {
            0 => EnemyType::RoseBuddies(RoseBuddies::new(reference)),
            1 => EnemyType::CopperSides(CopperSides::new(reference)),
            2 => EnemyType::Maestro(Maestro::new(reference)),
            3 => EnemyType::IronLotus(IronLotus::new()),
            4 => EnemyType::Bomb(Bomb::new(reference)),
            _ => EnemyType::Dummy,
        }
    }

    pub fn set_up (self, game: &mut Game) {
        match self {
            Self::RoseBuddies(enemy) => {},
            Self::CopperSides(enemy) => {},
            Self::Maestro(enemy) => {},
            Self::IronLotus(enemy) => {},
            Self::Bomb(enemy) => {enemy.set_up(game)},
            Self::Dummy => {},
        }
    }

    pub fn next_turn(self, game: &mut Game) -> i64 {
        match self {
            Self::RoseBuddies(enemy) => {enemy.next_turn()},
            Self::CopperSides(enemy) => {enemy.next_turn()},
            Self::Maestro(enemy) => {enemy.next_turn()},
            Self::IronLotus(enemy) => {6},
            Self::Bomb(enemy) => {enemy.next_turn()},
            Self::Dummy => {100},
        }
    }

    pub fn choose_action(self) -> ActionCard {
        match self {
            Self::RoseBuddies(enemy) => {enemy.choose_action()},
            Self::CopperSides(enemy) => {enemy.choose_action()},
            Self::Maestro(enemy) => {enemy.choose_action()},
            Self::IronLotus(enemy) => {enemy.choose_action()},
            Self::Bomb(enemy) => {enemy.choose_action()},
            Self::Dummy => {},
        }
    }

    pub fn message(self) -> Option<String> {
        match self {
            Self::RoseBuddies(enemy) => {enemy.message()},
            Self::CopperSides(enemy) => {enemy.message()},
            Self::Maestro(enemy) => {enemy.message()},
            Self::IronLotus(enemy) => {enemy.message()},
            Self::Bomb(enemy) => {enemy.message()},
            Self::Dummy => {None},
        }
    }
}
