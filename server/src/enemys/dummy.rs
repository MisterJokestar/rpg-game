use crate::{
    models::{Action, game::Game},
    enemys::{Enemy, EnemyType}
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
    fn get_type(&self) -> EnemyType {
        EnemyType::Dummy
    }

    fn get_max_health(&self, state: &mut Game) -> i64 {
        _ = state;
        100
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
