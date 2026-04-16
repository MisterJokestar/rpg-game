use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub complete: bool,
    pub win: Option<bool>,
    pub round: i32,
    pub player_state: PlayerState,
    pub enemy_state: EnemyState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub player_id: Uuid,
    pub character_id: Uuid,
    pub health: (i32, i32),
    pub damage_taken: i32,
    pub damage_healed: i32,
    pub damage_blocked: i32,
    pub damage_dodged: i32,
    pub damage_dealt:i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyState {
    pub health: (i32, i32),
    pub enemy_type: String, // Probably make into a enum later.
}
