use serde::{Deserialize, Serialize};

use crate::{
    models::Action,
    enemys::dummy::DummyEnemy,
};

pub mod dummy;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnemyType {
    Dummy,
}

pub trait Enemy: Send + Sync {
    fn get_type(&self) -> EnemyType;
    fn get_max_health(&self) -> i32;
    fn next_turn(&self) -> i32;
    fn choose_action(&self) -> Action;
}

pub fn get_enemy_by_type(enemy_type: EnemyType) -> Box<dyn Enemy> {
    match enemy_type {
        EnemyType::Dummy => Box::new(DummyEnemy::new()),
    }
}
