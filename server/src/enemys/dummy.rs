use crate::{
    models::Action,
    enemys::{Enemy, EnemyType}
};

// Dummy enemy is a stand in enemy, 
// on its action it will do nothing.

pub struct DummyEnemy {
    enemy_type: EnemyType,
    max_health: i32,
}

impl DummyEnemy {
    pub fn new() -> Self {
        DummyEnemy { 
            enemy_type: EnemyType::Dummy, 
            max_health: 100 
        }
    }
}

impl Enemy for DummyEnemy {
    fn get_type(&self) -> EnemyType {
        self.enemy_type
    }

    fn get_max_health(&self) -> i32 {
        self.max_health
    }

    // This will change.
    fn choose_action(&self) -> Action {
        Action::None
    }
}
