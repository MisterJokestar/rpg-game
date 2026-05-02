//! Player character model.
//!
//! A [`Character`] belongs to exactly one [`crate::models::user::User`] and
//! accumulates a list of games it has participated in. Combat stats are
//! stored in a nested [`Stats`] struct.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A player character stored in the database.
///
/// The `id` field is serialised as `_firestore_id` or `_id` depending on the
/// active database backend feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// Unique character identifier (UUID v7).
    #[cfg_attr(feature = "firestore", serde(rename = "_firestore_id"))]
    #[cfg_attr(feature = "mongodb", serde(rename = "_id"))]
    pub id: String,
    /// ID of the [`crate::models::user::User`] who owns this character.
    pub owner: String,
    /// Display name chosen by the player.
    pub name: String,
    /// Combat statistics that influence game-engine calculations.
    pub stats: Stats,
    /// IDs of all games this character has been used in.
    pub games: Vec<String>
}

/// Combat statistics for a [`Character`].
///
/// All values are used as *scaling factors* inside the game engine rather than
/// absolute numbers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    /// Scales attack damage output.
    pub power: i64,
    /// Determines how frequently the character acts (lower turn interval =
    /// faster).
    pub speed: i64,
    /// Scales block and heal amounts.
    pub defense: i64
}

/// Request body for the `POST /character/new` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCharacterRequest {
    /// ID of the owning user.
    pub player_id: String,
    /// Display name for the new character.
    pub character_name: String,
    /// Initial power stat.
    pub power: i64,
    /// Initial speed stat.
    pub speed: i64,
    /// Initial defense stat.
    pub defense: i64
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCharacterResponse {
    pub character_id: String,
}

impl Character {
    /// Construct a new character with a generated UUID v7 identifier.
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
