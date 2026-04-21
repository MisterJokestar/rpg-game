use serde::{Deserialize, Serialize};

use crate::{
    enemys::dummy::DummyEnemy, models::{Action, game::Game}
};

pub mod dummy;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnemyType {
    Dummy,
}

pub trait Enemy: Send + Sync {
    fn get_type(&self) -> EnemyType;
    fn get_max_health(&self, state: &mut Game) -> i64;
    fn next_turn(&self, state: &mut Game) -> i64;
    fn choose_action(&self, state: &mut Game) -> Action;
    fn after_players_turn(&self, state: &mut Game);
}

pub fn get_enemy_by_type(enemy_type: EnemyType) -> Box<dyn Enemy> {
    match enemy_type {
        EnemyType::Dummy => Box::new(DummyEnemy::new()),
    }
}
