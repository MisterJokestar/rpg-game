use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub secret: String,
    pub characters: Vec<Character>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    pub stats: Stats,
    pub games: Vec<Uuid>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub power: i32,
    pub speed: i32,
    pub defense: i32
}

#[derive(Debug, Deserialize)]
pub struct AddCharacterRequest {
    pub username: String,
    pub name: String,
    pub stats: Stats,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCharacterRequest {
    pub username: String,
    pub name: Option<String>, // Not required to update name
    pub stats: Option<Stats>, // Not required to update stats
}

impl User {
    pub fn add_character(mut self, req: AddCharacterRequest) -> Self {
        let new_char = Character {
            id: Uuid::new_v4(),
            name: req.name,
            stats: req.stats,
            games: Vec::new(),
        };
        self.characters.push(new_char);
        self
    }

    pub fn update_character(mut self, char_id: Uuid, req: UpdateCharacterRequest) -> Option<Self> {
        let char = self.characters.iter_mut().find(
            |c| c.id == char_id)?;
        if let Some(name) = req.name { char.name = name;}
        if let Some(stats) = req.stats { char.stats = stats;}
        Some(self)
    }
}
