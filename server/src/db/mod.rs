use async_trait::async_trait;
use crate::{error::AppError, models::{game::Game, user::Character}};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_secret_for_user(&self, user_id: String) -> Result<Option<String>, AppError>;
    async fn get_character_for_user(&self, user_id: String, character_id: String) -> Result<Option<Character>, AppError>;
}

#[async_trait]
pub trait GameRepository: Send + Sync {
    async fn get_game_by_id(&self, id: String) -> Result<Option<Game>, AppError>;
    async fn update_game_by_id(&self, id: String, game: &Game) -> Result<(), AppError>;
}

pub mod firestore;
