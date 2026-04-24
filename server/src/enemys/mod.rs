use rand::random_range;
use serde::{Deserialize, Serialize};

use crate::{
    enemys::{
        bomb::BombEnemy, copper_sides::CopperSidesEnemy, dummy::DummyEnemy, iron_lotus::IronLotusEnemy, maestro::MaestroEnemy, rose_buddies::RoseBuddiesEnemy
    }, 
    models::{
        Action, 
        game::{EnemyState, Game}
    }
};

pub mod dummy;
pub mod rose_buddies;
pub mod copper_sides;
pub mod maestro;
pub mod iron_lotus;
pub mod bomb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyType {
    Dummy,
    RoseBuddies,
    CopperSides,
    Maestro,
    IronLotus,
    Bomb,
    #[serde(other)]
    Unknown,
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
        EnemyType::CopperSides => Box::new(CopperSidesEnemy::new()),
        EnemyType::Maestro => Box::new(MaestroEnemy::new()),
        EnemyType::IronLotus => Box::new(IronLotusEnemy::new()),
        EnemyType::Bomb => Box::new(BombEnemy::new()),
        EnemyType::Unknown => Box::new(DummyEnemy::new()),
    }
}

pub fn get_random_enemy() -> Box<dyn Enemy> {
    // range is exclusive 0..1 -> 0 to but not including 1
    let rn = random_range(0..5);
    match rn {
        0 => Box::new(RoseBuddiesEnemy::new()),
        1 => Box::new(CopperSidesEnemy::new()),
        2 => Box::new(MaestroEnemy::new()),
        3 => Box::new(IronLotusEnemy::new()),
        4 => Box::new(BombEnemy::new()),
        _ => Box::new(DummyEnemy::new()),
    }
}
