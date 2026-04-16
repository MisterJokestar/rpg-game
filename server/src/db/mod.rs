use async_trait::async_trait;
use uuid::Uuid;
use crate::{error::AppError, models::{user::User, game::Game}};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, AppError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn update_user(&self, user: User) -> Result<User, AppError>;
}

#[async_trait]
pub trait GameRepository: Send + Sync {
    async fn create_game(&self, game: Game) -> Result<Game, AppError>;
    async fn get_game_by_id(&self, id: Uuid) -> Result<Option<Game>, AppError>;
    async fn update_game(&self, game: Game) -> Result<Game, AppError>;
    async fn list_games_for_player(&self, player_id: Uuid) -> Result<Vec<Game>, AppError>;
}

pub mod firestore;
pub mod mongodb;
