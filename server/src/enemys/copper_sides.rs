//! Copper Sides enemy.
//!
//! A tanky enemy with a strict attack/defend cycle. On its defend turn it
//! emits a "CHARGING!" message and its power permanently increases by 1,
//! making each subsequent attack hit harder. Power also scales with the
//! current round, so it is stronger in later rounds.
use crate::{
    enemys::{Enemy, EnemyType},
    models::{
        Action,
        game::{EnemyState, Game, Health}
    }
};

/// A heavily-armoured enemy that alternates attacking and defending.
///
/// **Attack/defend cycle:** attacks, then defends (gaining +1 power each
/// cycle). Initial power is boosted by the current round number, making this
/// enemy more dangerous in later rounds.
pub struct CopperSidesEnemy {
    power: i64,
    defense: i64,
    set_message: Option<String>
}

impl CopperSidesEnemy {
    /// Create a new CopperSidesEnemy with default stats.
    pub fn new() -> Self {
        CopperSidesEnemy {
            power: 1,
            defense: 6,
            set_message: None
        }
    }
}

impl Enemy for CopperSidesEnemy {
    fn get_new_state(&mut self) -> EnemyState {
        EnemyState {
            next_turn: None,
            state: 2,
            health: Health {
                current: 50,
                max: 50
            },
            block: 20,
            enemy_type: EnemyType::CopperSides
        }
    }

    fn next_turn(&mut self, state: &mut Game) -> i64 {
        // On first call, adjust initial power depending on the current round
        if state.enemy_state.state == 2 {
            self.power += state.round;
            state.enemy_state.state = 0;
        }
        8
    }

    fn choose_action(&mut self, state: &mut Game) -> Action {
        // Enemy attacks on state 0, then defends and charges on state 1
        if state.enemy_state.state == 0 {
            state.enemy_state.next_turn = Some(state.turn + 1);
            state.enemy_state.state = 1;
            Action::Attack(self.power)
        } else {
            state.enemy_state.state = 0;
            self.power += 1;
            self.set_message = Some(String::from("CHARGING!"));
            Action::Defend(self.defense)
        }
    }

    fn after_players_turn(&mut self, state: &mut Game, prev_action: &Action) {
        _ = state;
        _ = prev_action;
    }

    fn message(&mut self) -> Option<String> {
        let send = self.set_message.clone();
        self.set_message = None;
        send
    }
}
