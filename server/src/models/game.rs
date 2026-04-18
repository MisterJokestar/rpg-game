use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enemys::EnemyType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub complete: bool,
    pub win: Option<bool>,
    pub round: i32,
    pub turn: i32,
    pub player_state: PlayerState,
    pub enemy_state: EnemyState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub player_id: Uuid,
    pub character_id: Uuid,
    pub next_turn: Option<i32>,
    pub health: (i32, i32),
    pub block: i32,
    pub damage_taken: i32,
    pub damage_healed: i32,
    pub damage_blocked: i32,
    pub damage_dodged: i32,
    pub damage_dealt:i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyState {
    pub next_turn: Option<i32>,
    pub health: (i32, i32),
    pub block: i32,
    pub enemy_type: EnemyType,
}

pub trait Combatant {
    fn get_block(&self) -> i32;
    fn set_block(&mut self, new_block: i32);
    fn get_health(&self) -> (i32, i32);
    fn set_health(&mut self, new_health: (i32, i32));
}

impl Combatant for PlayerState {
    fn get_block(&self) -> i32 {
        self.block
    }

    fn set_block(&mut self, new_block: i32) {
        self.block = new_block;
    }

    fn get_health(&self) -> (i32, i32) {
        self.health
    }

    fn set_health(&mut self, new_health: (i32, i32)) {
        self.health = new_health;
    }
}

impl Combatant for EnemyState {
    fn get_block(&self) -> i32 {
        self.block
    }

    fn set_block(&mut self, new_block: i32) {
        self.block = new_block;
    }

    fn get_health(&self) -> (i32, i32) {
        self.health
    }

    fn set_health(&mut self, new_health: (i32, i32)) {
        self.health = new_health;
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum GameEvent {
    TurnResolved(Game),
    GameOver(Game),
    GameStopped(Game),
}

#[derive(Debug, Deserialize)]
pub struct NewGameRequest {
    pub player_id: Uuid,
    pub character_id: Uuid,
    pub max_health: i32,
}

impl Game {
    pub fn new(req: NewGameRequest) -> Self {
        Game {
    		id: Uuid::new_v4(),
    		complete: false,
    		win: None,
    		round: 1,
            turn: 0,
    		player_state: PlayerState {
    			player_id: Uuid::new_v4(),
    			character_id: Uuid::new_v4(),
                next_turn: None,
    			health: (req.max_health, req.max_health),
                block: 0,
    			damage_taken: 0,
    			damage_healed: 0,
    			damage_blocked: 0,
    			damage_dodged: 0,
    			damage_dealt: 0,
            },
    		enemy_state: EnemyState::new(),
        }
    }
}

impl EnemyState {
    fn new() -> Self {
        EnemyState {
            next_turn: None,
            health: (100, 100),
            block: 0,
            enemy_type: EnemyType::Dummy,
        }
    }
}
