//! Bomb enemy — a countdown detonator.
//!
//! The Bomb has only 1 HP and counts down 20 turns before detonating. If the
//! player attacks the Bomb before it detonates, it explodes immediately for 20
//! damage. When it detonates naturally (turn counter expires) it attacks for a
//! base power of 5, sets its own HP to 0, and is defeated.
use crate::{
    enemys::{Enemy, EnemyType}, models::{Action, game::{Combatant, EnemyState, Game, Health}}
};

/// A fragile enemy that detonates after 20 turns or when struck.
///
/// **Detonation mechanic:** waits 20 turns, then self-destructs with an
/// `Attack(5)` action (killing itself). If the player attacks at any point
/// before detonation, the bomb explodes immediately dealing 20 damage to the
/// player.
pub struct BombEnemy {
    set_message: Option<String>,
}

impl BombEnemy {
    /// Create a new BombEnemy.
    pub fn new() -> Self {
        BombEnemy {
            set_message: None,
        }
    }
}

impl Enemy for BombEnemy {
    fn get_new_state(&mut self) -> EnemyState {
        EnemyState {
            next_turn: None,
            state: 0,
            health: Health {
                current: 1,
                max: 1
            },
            block: 0,
            enemy_type: EnemyType::Bomb
        }
    }

    fn next_turn(&mut self, state: &mut Game) -> i64 {
        _ = state;
        self.set_message = Some(String::from("T-20 turns!"));
        20
    }

    fn choose_action(&mut self, state: &mut Game) -> Action {
        // Bomb self-destructs when its turn arrives
        state.enemy_state.health.current = 0;
        self.set_message = Some(String::from("KABOOM!"));
        Action::Attack(5)
    }

    fn after_players_turn(&mut self, state: &mut Game, prev_action: &Action) {
        // Bomb detonates immediately if the player attacks it
        if let Action::Attack(_) = prev_action {
            self.set_message = Some(String::from("KABOOM!"));
            state.player_state.deal_damage(20);
        };
    }

    fn message(&mut self) -> Option<String> {
        let send = self.set_message.clone();
        self.set_message = None;
        send
    }
}
