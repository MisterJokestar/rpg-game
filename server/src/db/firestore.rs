// use async_trait::async_trait;
use firestore::FirestoreDb;

use crate::{
    error::AppError,
};
// TODO: Replace with game/user Repository impl.
// use crate::{
//     db::Repository,
//     models::{CreateItemRequest, Item, UpdateItemRequest},
// };

const COLLECTION: &str = "items";

pub struct FirestoreRepository {
    db: FirestoreDb,
}

impl FirestoreRepository {
    pub async fn new(project_id: &str) -> Result<Self, AppError> {
        let db = FirestoreDb::new(project_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Firestore creates collections automatically on first write.
        // This log confirms the connection was established at startup.
        // Note: composite indexes (e.g. username + character queries) must be
        // created via the Firebase console or firestore.indexes.json — they
        // cannot be managed from application code.
        tracing::info!(
            "Connected to Firestore (project: {}). Collections 'users' and 'games' \
             will be created on first write.",
            project_id
        );

        Ok(Self { db })
    }
}

// TODO: Replace with game/user Repository impl.
// #[async_trait]
// impl Repository for FirestoreRepository {
//     async fn get_by_id(&self, id: &str) -> Result<Option<Item>, AppError> { ... }
//     async fn get_all(&self) -> Result<Vec<Item>, AppError> { ... }
//     async fn create(&self, req: CreateItemRequest) -> Result<Item, AppError> { ... }
//     async fn update(&self, id: &str, req: UpdateItemRequest) -> Result<Option<Item>, AppError> { ... }
//     async fn delete(&self, id: &str) -> Result<bool, AppError> { ... }
// }
