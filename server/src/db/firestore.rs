use async_trait::async_trait;
use firestore::{FirestoreDb, path, paths};

use crate::{
    db::{CharacterRepository, GameRepository, UserRepository},
    error::AppError,
    models::{
        character::{Character},
        game::Game,
        user::{User, UsernameOnly}
    },
};

const USER_COLLECTION: &str = "User";
const GAME_COLLECTION: &str = "Game";
const CHARACTER_COLLECTION: &str = "Character";

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
    async fn get_users(&self) -> Result<Vec<String>, AppError> {
        let users: Vec<UsernameOnly> = self.db
            .fluent()
            .select()
            .fields(paths!(User::username))
            .from(USER_COLLECTION)
            .obj::<UsernameOnly>()
            .query()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(users.into_iter().map(|u| u.username).collect())
    }

    async fn get_user_by_name(&self, username: String) -> Result<Option<User>, AppError> {
        let user: Vec<User> = self.db
            .fluent()
            .select()
            .from(USER_COLLECTION)
            .filter(|q| {
                q.field(path!(User::username)).eq(username.clone())
            })
            .obj::<User>()
            .query()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.into_iter().next())
    }

    async fn get_user_by_id(&self, user_id: String) -> Result<Option<User>, AppError> {
        let user: Option<User> = self.db
            .fluent()
            .select()
            .by_id_in(USER_COLLECTION)
            .obj::<User>()
            .one(&user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user)
    }

    async fn create_user(&self, user: &User) -> Result<(), AppError>{
        let _ = self.db
            .fluent()
            .insert()
            .into(USER_COLLECTION)
            .document_id(user.id.to_string())
            .object(user)
            .execute::<User>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<(), AppError> {
        let _ = self.db
            .fluent()
            .update()
            .in_col(USER_COLLECTION)
            .document_id(user.id.to_string())
            .object(user)
            .execute::<User>()
            .await
            .map_err(|e| AppError::Database(e.to_string()));

        Ok(())
    }

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
}

#[async_trait]
impl GameRepository for FirestoreRepository {
    async fn get_game(&self, id: String) -> Result<Option<Game>, AppError> {
        let result = self.db
            .fluent()
            .select()
            .by_id_in(GAME_COLLECTION)
            .obj::<Game>()
            .one(&id.to_string())
            .await
            .map_err(|e| AppError::Database(e.to_string()));

        result
    }

    async fn update_game(&self, id: String, game: &Game) -> Result<(), AppError> {
        let _ = self.db
            .fluent()
            .update()
            .in_col(GAME_COLLECTION)
            .document_id(&id)
            .object(game)
            .execute::<Game>()
            .await
            .map_err(|e| AppError::Database(e.to_string()));

        Ok(())
    }

    async fn create_game(&self, game: &Game) -> Result<(), AppError> {
        let _ = self.db
            .fluent()
            .insert()
            .into(GAME_COLLECTION)
            .document_id(&game.id)
            .object(game)
            .execute::<Game>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn get_all_games(&self) -> Result<Vec<Game>, AppError> {
        let games: Vec<Game> = self.db
            .fluent()
            .select()
            .from(GAME_COLLECTION)
            .obj::<Game>()
            .query()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(games)
    }
}

#[async_trait]
impl CharacterRepository for FirestoreRepository {
    async fn get_character(
        &self,
        character_id: String
    ) -> Result<Option<Character>, AppError> {
        let character: Option<Character> = self.db
            .fluent()
            .select()
            .by_id_in(CHARACTER_COLLECTION)
            .obj::<Character>()
            .one(&character_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(character)
    }

    async fn update_character(
        &self,
        character: &Character,
    ) -> Result<(), AppError> {
        let _ = self.db
            .fluent()
            .update()
            .in_col(CHARACTER_COLLECTION)
            .document_id(&character.id)
            .object(character)
            .execute::<Character>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn create_character(
        &self,
        character: &Character,
    ) -> Result<(), AppError> {
        let _ = self.db
            .fluent()
            .insert()
            .into(CHARACTER_COLLECTION)
            .document_id(&character.id)
            .object(character)
            .execute::<Character>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}
