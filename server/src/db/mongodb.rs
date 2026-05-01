//! MongoDB database backend.
//!
//! [`MongoRepository`] implements [`UserRepository`], [`GameRepository`], and
//! [`CharacterRepository`] using the official `mongodb` async driver. Enable
//! this backend by building with `--features mongodb`.
//!
//! All three domain collections (`User`, `Game`, `Character`) live in the
//! `rpg` database.
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Database};

use crate::{
    db::{CharacterRepository, GameRepository, UserRepository},
    error::AppError,
    models::{character::Character, game::Game, user::User},
};

const USER_COLLECTION: &str = "User";
const GAME_COLLECTION: &str = "Game";
const CHARACTER_COLLECTION: &str = "Character";

/// MongoDB-backed repository that satisfies all three repository traits.
///
/// A single instance is created at startup and shared (via `Arc`) across all
/// route handlers.
pub struct MongoRepository {
    db: Database,
}

impl MongoRepository {
    /// Connect to the MongoDB instance at `uri` and open the `rpg` database.
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Database`] if the MongoDB client cannot be created
    /// (e.g., malformed URI).
    pub async fn new(uri: &str) -> Result<Self, AppError> {
        let client = Client::with_uri_str(uri)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let db = client.database("rpg");
        tracing::info!("Connected to MongoDB (uri: {}).", uri);
        Ok(Self { db })
    }
}

#[async_trait]
impl UserRepository for MongoRepository {
    async fn get_users(&self) -> Result<Vec<String>, AppError> {
        let collection = self.db.collection::<User>(USER_COLLECTION);
        let cursor = collection
            .find(doc! {})
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let users: Vec<User> = cursor
            .try_collect()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(users.into_iter().map(|u| u.username).collect())
    }

    async fn get_user_by_name(&self, username: String) -> Result<Option<User>, AppError> {
        let collection = self.db.collection::<User>(USER_COLLECTION);
        collection
            .find_one(doc! { "username": username })
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn get_user_by_id(&self, user_id: String) -> Result<Option<User>, AppError> {
        let collection = self.db.collection::<User>(USER_COLLECTION);
        collection
            .find_one(doc! { "_id": user_id })
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn create_user(&self, user: &User) -> Result<(), AppError> {
        let collection = self.db.collection::<User>(USER_COLLECTION);
        collection
            .insert_one(user)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<(), AppError> {
        let collection = self.db.collection::<User>(USER_COLLECTION);
        collection
            .replace_one(doc! { "_id": &user.id }, user)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn get_secret_for_user(&self, user_id: String) -> Result<Option<String>, AppError> {
        let collection = self.db.collection::<User>(USER_COLLECTION);
        let user = collection
            .find_one(doc! { "_id": user_id })
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user.map(|u| u.secret))
    }
}

#[async_trait]
impl GameRepository for MongoRepository {
    async fn get_game(&self, id: String) -> Result<Option<Game>, AppError> {
        let collection = self.db.collection::<Game>(GAME_COLLECTION);
        collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn update_game(&self, id: String, game: &Game) -> Result<(), AppError> {
        let collection = self.db.collection::<Game>(GAME_COLLECTION);
        collection
            .replace_one(doc! { "_id": id }, game)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn create_game(&self, game: &Game) -> Result<(), AppError> {
        let collection = self.db.collection::<Game>(GAME_COLLECTION);
        collection
            .insert_one(game)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn get_all_games(&self) -> Result<Vec<Game>, AppError> {
        let collection = self.db.collection::<Game>(GAME_COLLECTION);
        let cursor = collection
            .find(doc! {})
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        cursor
            .try_collect()
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}

#[async_trait]
impl CharacterRepository for MongoRepository {
    async fn get_character(&self, character_id: String) -> Result<Option<Character>, AppError> {
        let collection = self.db.collection::<Character>(CHARACTER_COLLECTION);
        collection
            .find_one(doc! { "_id": character_id })
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn get_characters(&self, character_ids: Vec<String>) -> Result<Vec<Character>, AppError> {
        let collection = self.db.collection::<Character>(CHARACTER_COLLECTION);
        let cursor = collection
            .find(doc! { "_id": { "$in": &character_ids } })
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        cursor
            .try_collect()
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn update_character(&self, character: &Character) -> Result<(), AppError> {
        let collection = self.db.collection::<Character>(CHARACTER_COLLECTION);
        collection
            .replace_one(doc! { "_id": &character.id }, character)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn create_character(&self, character: &Character) -> Result<(), AppError> {
        let collection = self.db.collection::<Character>(CHARACTER_COLLECTION);
        collection
            .insert_one(character)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
