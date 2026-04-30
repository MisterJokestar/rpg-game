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
    let character = state.characters.get_character(character_id.clone()).await?
        .ok_or_else(|| AppError::NotFound(format!("Character with id, {}, Does not exist.", character_id)))?;
    Ok((StatusCode::OK, Json(character)))
}

// POST /character/update -> Updates a character by id from the database
pub async fn update_character(
    State(state): State<Arc<AppState>>,
    Json(updated_character): Json<Character>,
) -> Result<StatusCode, AppError> {
    state.characters.update_character(&updated_character).await?;
    Ok(StatusCode::OK)
}

// POST /character/new -> Creates a new character
pub async fn create_character(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateCharacterRequest>,
) -> Result<StatusCode, AppError> {
    let character = Character::new(
        request.player_id.clone(),
        request.character_name,
        request.power,
        request.speed,
        request.defense
    );
    let mut player = state.users.get_user_by_id(request.player_id.clone()).await?
        .ok_or_else(|| AppError::NotFound(format!("User with id, {}, Does Not Exist.", request.player_id)))?;
    state.characters.create_character(&character).await?;
    player.characters.push(character.id);
    state.users.update_user(&player).await?;
    Ok(StatusCode::CREATED)
}
