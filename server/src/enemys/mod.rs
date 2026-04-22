use rand::random_range;
use serde::{Deserialize, Serialize};

use crate::{
    enemys::{dummy::DummyEnemy, rose_buddies::RoseBuddiesEnemy}, 
    models::{
        Action, 
        game::{EnemyState, Game}}
};

pub mod dummy;
pub mod rose_buddies;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyType {
    Dummy,
    RoseBuddies,
}

pub trait Enemy: Send + Sync {
    fn get_new_state(&mut self) -> EnemyState;
    fn next_turn(&mut self, state: &mut Game) -> i64;
    fn choose_action(&mut self, state: &mut Game) -> Action;
    fn after_players_turn(&mut self, state: &mut Game, prev_action: &Action);
    fn message(&mut self) -> Option<String>;
}

pub fn get_enemy_by_type(enemy_type: EnemyType) -> Box<dyn Enemy> {
    match enemy_type {
        EnemyType::Dummy => Box::new(DummyEnemy::new()),
        EnemyType::RoseBuddies => Box::new(RoseBuddiesEnemy::new()),
    }
}

pub fn get_random_enemy() -> Box<dyn Enemy> {
    // range is exclusive 0..1 -> 0 to but not including 1
    let rn = random_range(0..1);
    match rn {
        0 => Box::new(RoseBuddiesEnemy::new()),
        _ => Box::new(DummyEnemy::new()),
    }
}
