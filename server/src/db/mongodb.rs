use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, to_bson},
    options::ReturnDocument,
    Client, Collection,
};

use crate::{
    db::Repository,
    error::AppError,
    models::{CreateItemRequest, Item, UpdateItemRequest},
};

const DEFAULT_DB_NAME: &str = "app_db";
const COLLECTION_NAME: &str = "items";

pub struct MongoRepository {
    collection: Collection<Item>,
}

impl MongoRepository {
    pub async fn new(uri: &str, db_name: Option<&str>) -> Result<Self, AppError> {
        let client = Client::with_uri_str(uri)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let db = client.database(db_name.unwrap_or(DEFAULT_DB_NAME));
        let collection: Collection<Item> = db.collection(COLLECTION_NAME);
        Ok(Self { collection })
    }
}

#[async_trait]
impl Repository for MongoRepository {
    async fn get_by_id(&self, id: &str) -> Result<Option<Item>, AppError> {
        self.collection
            .find_one(doc! { "id": id })
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn get_all(&self) -> Result<Vec<Item>, AppError> {
        let cursor = self
            .collection
            .find(doc! {})
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        cursor
            .try_collect::<Vec<Item>>()
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn create(&self, req: CreateItemRequest) -> Result<Item, AppError> {
        let item = Item::new(req);
        self.collection
            .insert_one(&item)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(item)
    }

    async fn update(&self, id: &str, req: UpdateItemRequest) -> Result<Option<Item>, AppError> {
        let mut set_doc = doc! {};

        if let Some(name) = req.name {
            set_doc.insert("name", name);
        }
        if let Some(data) = req.data {
            let bson_data = to_bson(&data).map_err(|e| AppError::Serialization(e.to_string()))?;
            set_doc.insert("data", bson_data);
        }

        // Nothing to update — return current state.
        if set_doc.is_empty() {
            return self.get_by_id(id).await;
        }

        self.collection
            .find_one_and_update(doc! { "id": id }, doc! { "$set": set_doc })
            .return_document(ReturnDocument::After)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn delete(&self, id: &str) -> Result<bool, AppError> {
        let result = self
            .collection
            .delete_one(doc! { "id": id })
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.deleted_count > 0)
    }
}
