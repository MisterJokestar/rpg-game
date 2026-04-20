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
    fn set_block(&mut self, block: i32);
    fn reset_block(&mut self);
    fn take_damage(&mut self, damage: i32) -> i32;
    fn deal_damage(&mut self, damage: i32);
    fn heal_damage(&mut self, healing: i32);
}

impl Combatant for PlayerState {
    fn set_block(&mut self, block: i32) {
        self.block = block;
    }

    fn reset_block(&mut self) {
        self.block = 0;
    }

    fn take_damage(&mut self, damage: i32) -> i32 {
        let attack_dmg = damage - self.block;
        if attack_dmg > 0 {
            self.damage_blocked += self.block;
            self.block = 0;
            let (total_hp, mut current_hp) = self.health;
            current_hp = current_hp - attack_dmg;
            let mut damage_taken;
            if current_hp <= 0 {
                self.health = (total_hp, 0);
                damage_taken = attack_dmg + current_hp;
            } else {
                self.health = (total_hp, current_hp);
                damage_taken = attack_dmg;
            }
            self.damage_taken += damage_taken;
            return damage_taken
        } else {
            self.damage_blocked = damage + attack_dmg;
            self.block = (-1 * attack_dmg);
            return 0;
        }
    }

    fn deal_damage(&mut self, damage: i32) {
        self.damage_dealt += damage;
    }

    fn heal_damage(&mut self, healing: i32) {
        let (total_hp, mut current_hp) = self.health;
        if healing + current_hp > total_hp {
            self.damage_healed += total_hp - current_hp;
            self.health = (total_hp, total_hp);
        } else {
            self.damage_healed += healing;
            self.health = (total_hp, current_hp + healing);
        }
    }
}

impl Combatant for EnemyState {
    fn set_block(&mut self, block: i32) {
        self.block = block;
    }

    fn reset_block(&mut self) {
        self.block = 0;
    }

    fn take_damage(&mut self, damage: i32) -> i32 {
        let attack_dmg = damage - self.block;
        if attack_dmg > 0 {
            self.block = 0;
            let (total_hp, mut current_hp) = self.health;
            current_hp = current_hp - attack_dmg;
            let mut damage_taken;
            if current_hp <= 0 {
                self.health = (total_hp, 0);
                damage_taken = attack_dmg + current_hp;
            } else {
                self.health = (total_hp, current_hp);
                damage_taken = attack_dmg;
            }
            return damage_taken
        } else {
            self.block = (-1 * attack_dmg);
            return 0;
        }
    }

    fn deal_damage(&mut self, damage: i32) {
        return
    }

    fn heal_damage(&mut self, healing: i32) {
        let (total_hp, mut current_hp) = self.health;
        if healing + current_hp > total_hp {
            self.health = (total_hp, total_hp);
        } else {
            self.health = (total_hp, current_hp + healing);
        }
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
