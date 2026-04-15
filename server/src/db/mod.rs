use async_trait::async_trait;

use crate::{
    error::AppError,
    models::{CreateItemRequest, Item, UpdateItemRequest},
};

/// Core database abstraction. Both the Firestore and MongoDB backends implement
/// this trait, so the rest of the application never touches a concrete type.
#[async_trait]
pub trait Repository: Send + Sync {
    async fn get_by_id(&self, id: &str) -> Result<Option<Item>, AppError>;
    async fn get_all(&self) -> Result<Vec<Item>, AppError>;
    async fn create(&self, req: CreateItemRequest) -> Result<Item, AppError>;
    async fn update(&self, id: &str, req: UpdateItemRequest) -> Result<Option<Item>, AppError>;
    async fn delete(&self, id: &str) -> Result<bool, AppError>;
}

pub mod firestore;
pub mod mongodb;
