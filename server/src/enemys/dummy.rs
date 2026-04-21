use crate::{
    enemys::{Enemy, EnemyType}, models::{Action, game::{EnemyState, Game, Health}}
};

// Dummy enemy is a stand in enemy, 
// on its action it will do nothing.

pub struct DummyEnemy {
}

impl DummyEnemy {
    pub fn new() -> Self {
        DummyEnemy { 
        }
    }
}

impl Enemy for DummyEnemy {
    fn get_new_state(&self) -> EnemyState {
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

    fn next_turn(&self, state: &mut Game) -> i64 {
        _ = state;
        100
    }

    // This will change.
    fn choose_action(&self, state: &mut Game) -> Action {
        _ = state;
        Action::None
    }

    fn after_players_turn(&self, state: &mut Game) {
        _ = state;
    }
}
