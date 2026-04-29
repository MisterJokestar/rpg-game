use async_trait::async_trait;
use crate::{
    error::AppError, 
    models::{game::Game, character::Character}
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, username: String, password_hash: String) -> Result<(), AppError>;
    async fn get_user_hash(&self, username: String) -> Result<Option<String>, AppError>;
    async fn get_secret_for_user(&self, user_id: String) -> Result<Option<String>, AppError>;
    async fn get_character_for_user(&self, user_id: String, character_id: String) -> Result<Option<Character>, AppError>;
    async fn update_character_for_user(&self, user_id: String, character: &Character) -> Result<(), AppError>;
    async fn create_character_for_user(&self, user_id: String, character: &Character) -> Result<(), AppError>;
}

#[async_trait]
pub trait GameRepository: Send + Sync {
    async fn get_game_by_id(&self, id: String) -> Result<Option<Game>, AppError>;
    async fn update_game_by_id(&self, id: String, game: &Game) -> Result<(), AppError>;
    async fn create_game(&self, game: &Game) -> Result<(), AppError>;
    async fn get_all_games(&self) -> Result<Vec<Game>, AppError>;
}

pub mod firestore;
