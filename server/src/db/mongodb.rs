// use async_trait::async_trait;
// use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    options::IndexOptions,
    Client, IndexModel,
};

use crate::{
    error::AppError,
};
// TODO: Replace with game/user Repository impl.
// use crate::{
//     db::Repository,
//     models::{CreateItemRequest, Item, UpdateItemRequest},
// };

const DEFAULT_DB_NAME: &str = "app_db";
const COLLECTION_NAME: &str = "items";
const USERS_COLLECTION: &str = "users";

pub struct MongoRepository {
    // TODO: Replace Collection<Item> with game/user collections.
    // collection: Collection<Item>,
}

impl MongoRepository {
    pub async fn new(uri: &str, db_name: Option<&str>) -> Result<Self, AppError> {
        let client = Client::with_uri_str(uri)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let db = client.database(db_name.unwrap_or(DEFAULT_DB_NAME));

        // Ensure unique index on username for the users collection.
        let users = db.collection::<mongodb::bson::Document>(USERS_COLLECTION);
        let index = IndexModel::builder()
            .keys(doc! { "username": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();
        users
            .create_index(index)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        tracing::info!("MongoDB indexes ensured on '{}'", USERS_COLLECTION);

        // TODO: Initialize game/user collections here.
        // let collection: Collection<Item> = db.collection(COLLECTION_NAME);
        Ok(Self {})
    }
}

// TODO: Replace with game/user Repository impl.
// #[async_trait]
// impl Repository for MongoRepository {
//     async fn get_by_id(&self, id: &str) -> Result<Option<Item>, AppError> { ... }
//     async fn get_all(&self) -> Result<Vec<Item>, AppError> { ... }
//     async fn create(&self, req: CreateItemRequest) -> Result<Item, AppError> { ... }
//     async fn update(&self, id: &str, req: UpdateItemRequest) -> Result<Option<Item>, AppError> { ... }
//     async fn delete(&self, id: &str) -> Result<bool, AppError> { ... }
// }
