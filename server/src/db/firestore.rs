use async_trait::async_trait;
use firestore::FirestoreDb;
use uuid::Uuid;

use crate::{
    db::{UserRepository, GameRepository},
    error::AppError,
    models::{user::User, game::Game},
};

const USER_COLLECTION: &str = "user";
const GAME_COLLECTION: &str = "game";

pub struct FirestoreRepository {
    db: FirestoreDb,
}

impl FirestoreRepository {
    pub async fn new(project_id: &str) -> Result<Self, AppError> {
        let db = FirestoreDb::new(project_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Firestore creates collections automatically on first write.
        // Note: composite indexes (e.g. player_state.player_id for list_games_for_player)
        // must be created via the Firebase console or firestore.indexes.json.
        tracing::info!(
            "Connected to Firestore (project: {}). Collections '{}' and '{}' \
             will be created on first write.",
            project_id,
            USER_COLLECTION,
            GAME_COLLECTION,
        );

        Ok(Self { db })
    }
}

#[async_trait]
impl UserRepository for FirestoreRepository {
    async fn create_user(&self, user: User) -> Result<User, AppError> {
        self.db
            .fluent()
            .insert()
            .into(USER_COLLECTION)
            .document_id(&user.id.to_string())
            .object(&user)
            .execute::<User>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        self.db
            .fluent()
            .select()
            .by_id_in(USER_COLLECTION)
            .obj::<User>()
            .one(&id.to_string())
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let results: Vec<User> = self.db
            .fluent()
            .select()
            .from(USER_COLLECTION)
            .filter(|q| q.field("username").eq(username))
            .obj::<User>()
            .query()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(results.into_iter().next())
    }

    async fn update_user(&self, user: User) -> Result<User, AppError> {
        self.db
            .fluent()
            .update()
            .in_col(USER_COLLECTION)
            .document_id(&user.id.to_string())
            .object(&user)
            .execute::<User>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user)
    }
}

#[async_trait]
impl GameRepository for FirestoreRepository {
    async fn create_game(&self, game: Game) -> Result<Game, AppError> {
        self.db
            .fluent()
            .insert()
            .into(GAME_COLLECTION)
            .document_id(&game.id.to_string())
            .object(&game)
            .execute::<Game>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(game)
    }

    async fn get_game_by_id(&self, id: Uuid) -> Result<Option<Game>, AppError> {
        self.db
            .fluent()
            .select()
            .by_id_in(GAME_COLLECTION)
            .obj::<Game>()
            .one(&id.to_string())
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn update_game(&self, game: Game) -> Result<Game, AppError> {
        self.db
            .fluent()
            .update()
            .in_col(GAME_COLLECTION)
            .document_id(&game.id.to_string())
            .object(&game)
            .execute::<Game>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(game)
    }

    async fn list_games_for_player(&self, player_id: Uuid) -> Result<Vec<Game>, AppError> {
        self.db
            .fluent()
            .select()
            .from(GAME_COLLECTION)
            .filter(|q| q.field("player_state.player_id").eq(player_id.to_string()))
            .obj::<Game>()
            .query()
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
