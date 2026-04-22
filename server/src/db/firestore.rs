use async_trait::async_trait;
use firestore::FirestoreDb;

use crate::{
    db::{GameRepository, UserRepository},
    error::AppError,
    models::{game::Game, user::{Character, User}},
};

const USER_COLLECTION: &str = "User";
const GAME_COLLECTION: &str = "Game";

pub struct FirestoreRepository {
    db: FirestoreDb,
}

impl FirestoreRepository {
    pub async fn new(project_id: &str) -> Result<Self, AppError> {
        let db = FirestoreDb::new(project_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        tracing::info!(
            "Connected to Firestore (project: {}).",
            project_id
        );

        Ok(Self { db })
    }
}

#[async_trait]
impl UserRepository for FirestoreRepository {
    async fn get_secret_for_user(&self, user_id: String) -> Result<Option<String>, AppError> {
        let user: Option<User> = self.db
            .fluent()
            .select()
            .by_id_in(USER_COLLECTION)
            .obj::<User>()
            .one(&user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.map(|u| u.secret))
    }

    async fn get_character_for_user(&self, user_id: String, char_id: String) -> Result<Option<Character>, AppError> {
        let user: Option<User> = self.db
            .fluent()
            .select()
            .by_id_in(USER_COLLECTION)
            .obj::<User>()
            .one(&user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.and_then(|u| u.characters.into_iter().find(|c| c.id == char_id)))
    }
}

#[async_trait]
impl GameRepository for FirestoreRepository {
    async fn get_game_by_id(&self, id: String) -> Result<Option<Game>, AppError> {
        let result = self.db
            .fluent()
            .select()
            .by_id_in(GAME_COLLECTION)
            .obj::<Game>()
            .one(&id.to_string())
            .await
            .map_err(|e| AppError::Database(e.to_string()));
        tracing::debug!("get_game_by_id({id}) => {:?}", result);
        result
    }

    async fn update_game_by_id(&self, id: String, game: &Game) -> Result<(), AppError> {
        let _ = self.db
            .fluent()
            .update()
            .in_col(GAME_COLLECTION)
            .document_id(&id)
            .object(game)
            .execute::<Game>()
            .await
            .map_err(|e| AppError::Database(e.to_string()));
        tracing::debug!("update_game_by_id({id}) => ok");
        Ok(())
    }
}
