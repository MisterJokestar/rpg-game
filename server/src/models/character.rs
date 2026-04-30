use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    #[serde(rename = "_firestore_id")]
    pub id: String,
    pub owner: String,
    pub name: String,
    pub stats: Stats,
    pub games: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub power: i64,
    pub speed: i64,
    pub defense: i64
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCharacterRequest {
    pub player_id: String,
    pub character_name: String,
    pub power: i64,
    pub speed: i64,
    pub defense: i64
}

impl Character {
    pub fn new(
        owner: String,
        name: String,
        power: i64,
        speed: i64,
        defense: i64
    ) -> Self {
        Character {
            id: Uuid::now_v7().to_string(),
            owner,
            name,
            stats: Stats {
                power,
                speed,
                defense,
            },
            games: Vec::new()
        }
    }
}
