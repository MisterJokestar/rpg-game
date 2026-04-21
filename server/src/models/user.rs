use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub secret: String,
    pub characters: Vec<Character>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
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
