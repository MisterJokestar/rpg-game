use crate::{
    models::Action,
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

    fn get_max_health(&self) -> i64 {
        100
    }

    fn next_turn(&self) -> i64 {
        100
    }

    // This will change.
    fn choose_action(&self) -> Action {
        Action::None
    }
}
