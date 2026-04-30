use async_trait::async_trait;
use crate::{
    error::AppError, 
    models::{game::Game, character::Character, user::User}
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_users(&self) -> Result<Vec<String>, AppError>;
    async fn get_user(&self, username: String) -> Result<Option<User>, AppError>;
    async fn create_user(&self, user: &User) -> Result<(), AppError>;
    async fn update_user(&self, user: &User) -> Result<(), AppError>;
    async fn get_secret_for_user(&self, user_id: String) -> Result<Option<String>, AppError>;
}

#[async_trait]
pub trait GameRepository: Send + Sync {
    async fn get_game(&self, id: String) -> Result<Option<Game>, AppError>;
    async fn update_game(&self, id: String, game: &Game) -> Result<(), AppError>;
    async fn create_game(&self, game: &Game) -> Result<(), AppError>;
    async fn get_all_games(&self) -> Result<Vec<Game>, AppError>;
}

#[async_trait]
pub trait CharacterRepository: Send + Sync {
    async fn get_character(&self, character_id: String) -> Result<Option<Character>, AppError>;
    async fn update_character(&self, character: &Character) -> Result<(), AppError>;
    async fn create_character(&self, character: &Character) -> Result<(), AppError>;
}

pub mod firestore;
