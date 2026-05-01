//! Dummy enemy — a passive stand-in used as a placeholder and fallback.
//!
//! The Dummy never takes an action, always waits 100 turns before its next
//! turn, and does not react to the player. It is returned by
//! [`crate::enemys::get_enemy_by_type`] for [`crate::enemys::EnemyType::Unknown`].
use crate::{
    enemys::{Enemy, EnemyType}, models::{Action, game::{EnemyState, Game, Health}}
};

/// A passive enemy that takes no action.
///
/// Useful as a safe default and for testing the game loop without combat.
pub struct DummyEnemy {
}

impl DummyEnemy {
    /// Create a new DummyEnemy.
    pub fn new() -> Self {
        DummyEnemy {
        }
    }
}

impl Enemy for DummyEnemy {
    fn get_new_state(&mut self) -> EnemyState {
        EnemyState {
            next_turn: None,
            state: 0,
            health: Health {
                current: 100,
                max: 100
            },
            block: 0,
            enemy_type: EnemyType::Dummy
        }
    }

    fn next_turn(&mut self, state: &mut Game) -> i64 {
        _ = state;
        100
    }

    fn choose_action(&mut self, state: &mut Game) -> Action {
        _ = state;
        Action::None
    }

    fn after_players_turn(&mut self, state: &mut Game, prev_action: &Action) {
        _ = state;
        _ = prev_action;
    }

    fn message(&mut self) -> Option<String> {
        None
    }
}
