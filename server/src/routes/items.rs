use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::{
    error::AppError,
    models::{CreateItemRequest, Item, UpdateItemRequest},
    state::AppState,
};

pub async fn list_items(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Item>>, AppError> {
    let items = state.db.get_all().await?;
    Ok(Json(items))
}

pub async fn get_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Item>, AppError> {
    state
        .db
        .get_by_id(&id)
        .await?
        .map(Json)
        .ok_or_else(|| AppError::NotFound(format!("Item '{}' not found", id)))
}

pub async fn create_item(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<Item>), AppError> {
    let item = state.db.create(body).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateItemRequest>,
) -> Result<Json<Item>, AppError> {
    state
        .db
        .update(&id, body)
        .await?
        .map(Json)
        .ok_or_else(|| AppError::NotFound(format!("Item '{}' not found", id)))
}

pub async fn delete_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let deleted = state.db.delete(&id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("Item '{}' not found", id)))
    }
}
