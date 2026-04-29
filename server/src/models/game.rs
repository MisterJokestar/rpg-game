use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::enemys::{EnemyType, get_random_enemy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_firestore_id")]
    pub id: String,
    pub complete: bool,
    pub win: Option<bool>,
    pub round: i64,
    pub turn: i64,
    pub player_state: PlayerState,
    pub enemy_state: EnemyState,
    #[serde(default)]
    pub enemies_defeated: HashMap<EnemyType, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub player_id: String,
    pub character_id: String,
    pub next_turn: Option<i64>,
    pub health: Health,
    pub block: i64,
    pub damage_taken: i64,
    pub damage_healed: i64,
    pub damage_blocked: i64,
    pub damage_dodged: i64,
    pub damage_dealt:i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyState {
    pub next_turn: Option<i64>,
    pub state: i64,
    pub health: Health,
    pub block: i64,
    pub enemy_type: EnemyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: i64,
    pub max: i64,
}

pub trait Combatant {
    fn set_block(&mut self, block: i64);
    fn reset_block(&mut self);
    fn take_damage(&mut self, damage: i64) -> i64;
    fn deal_damage(&mut self, damage: i64);
    fn heal_damage(&mut self, healing: i64);
}

impl Combatant for PlayerState {
    fn set_block(&mut self, block: i64) {
        self.block = block;
    }

    fn reset_block(&mut self) {
        self.block = 0;
    }

    fn take_damage(&mut self, damage: i64) -> i64 {
        // get damage after block
        let attack_dmg = damage - self.block;
        // there is unblocked damage.
        if attack_dmg > 0 {
            self.damage_blocked += self.block;
            self.block = 0;
            let mut current_hp = self.health.current;
            // HP after unblocked damage applied.
            current_hp -= attack_dmg;
            let damage_taken;
            // If HP is reduced to 0, excess damage not logged
            if current_hp <= 0 {
                self.health.current = 0;
                damage_taken = attack_dmg + current_hp;
            } else {
                self.health.current = current_hp;
                damage_taken = attack_dmg;
            }
            // Log damage taken.
            self.damage_taken += damage_taken;
            // returns damage taken.
            damage_taken
        } else {
            // All damage was blocked.
            self.damage_blocked = damage + attack_dmg;
            // remaining block is kept.
            self.block = -attack_dmg;
            0
        }
    }

    fn deal_damage(&mut self, damage: i64) {
        // just for logging purposes
        self.damage_dealt += damage;
    }

    fn heal_damage(&mut self, healing: i64) {
        // Healing shouldn't overflow, caps at total_hp
        if healing + self.health.current > self.health.max {
            self.damage_healed += self.health.max - self.health.current;
            self.health.current = self.health.max;
        } else {
            self.damage_healed += healing;
            self.health.current += healing;
        }
    }
}

// Same as impl for PlayerState, but without logging.
impl Combatant for EnemyState {
    fn set_block(&mut self, block: i64) {
        self.block = block;
    }

    fn reset_block(&mut self) {
        self.block = 0;
    }

    fn take_damage(&mut self, damage: i64) -> i64 {
        let attack_dmg = damage - self.block;
        if attack_dmg > 0 {
            self.block = 0;
            let mut current_hp = self.health.current;
            current_hp -= attack_dmg;
            let damage_taken;
            if current_hp <= 0 {
                self.health.current = 0;
                damage_taken = attack_dmg + current_hp;
            } else {
                self.health.current = current_hp;
                damage_taken = attack_dmg;
            }
            damage_taken
        } else {
            self.block = -attack_dmg;
            0
        }
    }

    fn deal_damage(&mut self, damage: i64) {
        _ = damage;
    }

    fn heal_damage(&mut self, healing: i64) {
        if healing + self.health.current > self.health.max {
            self.health.current = self.health.max;
        } else {
            self.health.current += healing;
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum GameEvent {
    TurnResolved(Game),
    GameOver(Game),
    GameStopped(Game),
    GameMessage(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct SequencedEvent {
    pub seq: u64,
    pub event: GameEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGameRequest {

}

impl Game {
    pub fn set_up(&mut self) {
        self.player_state.next_turn = None;
        self.player_state.health = Health {
            current: 100,
            max: 100
        };
        self.enemy_state = get_random_enemy().get_new_state();
    }
}
