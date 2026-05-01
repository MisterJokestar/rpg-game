//! Database repository traits and backend module declarations.
//!
//! Three traits define the persistence contract for the three core domain
//! objects: [`UserRepository`], [`GameRepository`], and
//! [`CharacterRepository`]. Concrete implementations live in the
//! feature-gated sub-modules `firestore` and `mongodb`.
//!
//! Route handlers depend only on the traits, keeping them agnostic of the
//! chosen backend.
use async_trait::async_trait;
use crate::{
    error::AppError,
    models::{game::Game, character::Character, user::User}
};

/// Persistence operations for user accounts.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Return the usernames of all registered accounts.
    async fn get_users(&self) -> Result<Vec<String>, AppError>;
    /// Find a user by their username, returning `None` if not found.
    async fn get_user_by_name(&self, username: String) -> Result<Option<User>, AppError>;
    /// Find a user by their unique ID, returning `None` if not found.
    async fn get_user_by_id(&self, username: String) -> Result<Option<User>, AppError>;
    /// Insert a new user record into the database.
    async fn create_user(&self, user: &User) -> Result<(), AppError>;
    /// Replace an existing user record (matched by ID) with the provided data.
    async fn update_user(&self, user: &User) -> Result<(), AppError>;
    /// Retrieve only the session secret for a given user ID, or `None` if the
    /// user does not exist.
    async fn get_secret_for_user(&self, user_id: String) -> Result<Option<String>, AppError>;
}

/// Persistence operations for game records.
#[async_trait]
pub trait GameRepository: Send + Sync {
    /// Fetch a game by its unique ID, returning `None` if not found.
    async fn get_game(&self, id: String) -> Result<Option<Game>, AppError>;
    /// Replace an existing game record (matched by `id`) with `game`.
    async fn update_game(&self, id: String, game: &Game) -> Result<(), AppError>;
    /// Insert a new game record into the database.
    async fn create_game(&self, game: &Game) -> Result<(), AppError>;
    /// Return all game records in the database.
    async fn get_all_games(&self) -> Result<Vec<Game>, AppError>;
}

/// Persistence operations for player characters.
#[async_trait]
pub trait CharacterRepository: Send + Sync {
    /// Fetch a character by its unique ID, returning `None` if not found.
    async fn get_character(&self, character_id: String) -> Result<Option<Character>, AppError>;
    /// Replace an existing character record (matched by ID) with the provided
    /// data.
    async fn update_character(&self, character: &Character) -> Result<(), AppError>;
    /// Insert a new character record into the database.
    async fn create_character(&self, character: &Character) -> Result<(), AppError>;
}

#[cfg(feature = "firestore")]
pub mod firestore;

#[cfg(feature = "mongodb")]
pub mod mongodb;
