use async_trait::async_trait;
use firestore::FirestoreDb;

use crate::{
    db::Repository,
    error::AppError,
    models::{CreateItemRequest, Item, UpdateItemRequest},
};

const COLLECTION: &str = "items";

pub struct FirestoreRepository {
    db: FirestoreDb,
}

impl FirestoreRepository {
    pub async fn new(project_id: &str) -> Result<Self, AppError> {
        let db = FirestoreDb::new(project_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(Self { db })
    }
}

#[async_trait]
impl Repository for FirestoreRepository {
    async fn get_by_id(&self, id: &str) -> Result<Option<Item>, AppError> {
        self.db
            .fluent()
            .select()
            .by_id_in(COLLECTION)
            .obj::<Item>()
            .one(id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn get_all(&self) -> Result<Vec<Item>, AppError> {
        self.db
            .fluent()
            .select()
            .from(COLLECTION)
            .obj::<Item>()
            .query()
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn create(&self, req: CreateItemRequest) -> Result<Item, AppError> {
        let item = Item::new(req);
        self.db
            .fluent()
            .insert()
            .into(COLLECTION)
            .document_id(&item.id)
            .object(&item)
            .execute::<Item>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(item)
    }

    async fn update(&self, id: &str, req: UpdateItemRequest) -> Result<Option<Item>, AppError> {
        let Some(mut item) = self.get_by_id(id).await? else {
            return Ok(None);
        };

        if let Some(name) = req.name {
            item.name = name;
        }
        if let Some(data) = req.data {
            item.data = data;
        }

        self.db
            .fluent()
            .update()
            .in_col(COLLECTION)
            .document_id(id)
            .object(&item)
            .execute::<Item>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(Some(item))
    }

    async fn delete(&self, id: &str) -> Result<bool, AppError> {
        if self.get_by_id(id).await?.is_none() {
            return Ok(false);
        }

        self.db
            .fluent()
            .delete()
            .from(COLLECTION)
            .document_id(id)
            .execute()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(true)
    }
}
