use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    options::IndexOptions,
    Collection, Client, IndexModel,
};
use uuid::Uuid;

use crate::{
    db::{UserRepository, GameRepository},
    error::AppError,
    models::{user::User, game::Game},
};

const DEFAULT_DB_NAME: &str = "app_db";
const USERS_COLLECTION: &str = "user";
const GAMES_COLLECTION: &str = "game";

pub struct MongoRepository {
    users: Collection<User>,
    games: Collection<Game>,
}

impl MongoRepository {
    pub async fn new(uri: &str, db_name: Option<&str>) -> Result<Self, AppError> {
        let client = Client::with_uri_str(uri)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let db = client.database(db_name.unwrap_or(DEFAULT_DB_NAME));

        let users: Collection<User> = db.collection(USERS_COLLECTION);
        let games: Collection<Game> = db.collection(GAMES_COLLECTION);

        // Unique index on username for login lookups.
        let username_index = IndexModel::builder()
            .keys(doc! { "username": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();
        users
            .create_index(username_index)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Index for list_games_for_player queries.
        let player_index = IndexModel::builder()
            .keys(doc! { "player_state.player_id": 1 })
            .build();
        games
            .create_index(player_index)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        tracing::info!("MongoDB indexes ensured on '{}' and '{}'", USERS_COLLECTION, GAMES_COLLECTION);

        Ok(Self { users, games })
    }
}

#[async_trait]
impl UserRepository for MongoRepository {
    async fn create_user(&self, user: User) -> Result<User, AppError> {
        self.users
            .insert_one(&user)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        self.users
            .find_one(doc! { "_id": id.to_string() })
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        self.users
            .find_one(doc! { "username": username })
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn update_user(&self, user: User) -> Result<User, AppError> {
        let id = user.id.to_string();
        self.users
            .replace_one(doc! { "_id": id }, &user)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user)
    }
}

#[async_trait]
impl GameRepository for MongoRepository {
    async fn create_game(&self, game: Game) -> Result<Game, AppError> {
        self.games
            .insert_one(&game)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(game)
    }

    async fn get_game_by_id(&self, id: Uuid) -> Result<Option<Game>, AppError> {
        self.games
            .find_one(doc! { "_id": id.to_string() })
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn update_game(&self, game: Game) -> Result<Game, AppError> {
        let id = game.id.to_string();
        self.games
            .replace_one(doc! { "_id": id }, &game)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(game)
    }

    async fn list_games_for_player(&self, player_id: Uuid) -> Result<Vec<Game>, AppError> {
        let cursor = self.games
            .find(doc! { "player_state.player_id": player_id.to_string() })
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        cursor
            .try_collect()
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
