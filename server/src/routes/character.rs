//! character.rs
//!
//! Route handlers for characters.
use std::sync::Arc;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode
};
use crate::{
    error::AppError,
    models::character::{Character, CreateCharacterRequest},
    state::AppState
};

// GET /character/:character_id -> Grabs a character by id from the database
pub async fn get_character(
    State(state): State<Arc<AppState>>,
    Path(character_id): Path<String>,
) -> Result<(StatusCode, Json<Character>), AppError> {
    Err(AppError::NotImplemented)
}

// POST /character/:character_id -> Updates a character by id from the database
pub async fn update_character(
    State(state): State<Arc<AppState>>,
    Path(character_id): Path<String>,
    Json(updated_character): Json<Character>,
) -> Result<StatusCode, AppError> {
    Err(AppError::NotImplemented)
}

// POST /character/new -> Creates a new character
pub async fn create_character(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateCharacterRequest>,
) -> Result<StatusCode, AppError> {
    Err(AppError::NotImplemented)
}
