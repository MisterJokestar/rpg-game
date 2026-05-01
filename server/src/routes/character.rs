//! Character route handlers.
//!
//! All three handlers require authentication via the `Authorization` header.
//!
//! - `GET  /character/:character_id` — fetch a character by ID.
//! - `POST /character/update`         — replace an existing character record.
//! - `POST /character/new`            — create a new character and link it to
//!   a user.
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

/// `GET /character/all/:user_id` — fetch user's characters.
///
/// # Errors
///
/// - [`AppError::NotFound`] if no user exists with the given ID.
/// - [`AppError::Database`] if the database read fails.
pub async fn get_characters(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<(StatusCode, Json<Vec<Character>>), AppError> {
    let user = state.users.get_user_by_id(user_id.clone()).await?
        .ok_or_else(|| AppError::NotFound(format!("User with id, {}, Does not exist.", user_id)))?;
    let characters = state.characters.get_characters(user.characters).await?;
    Ok((StatusCode::OK, Json(characters)))
}

/// `GET /character/:character_id` — fetch a character by its unique ID.
///
/// # Errors
///
/// - [`AppError::NotFound`] if no character exists with the given ID.
/// - [`AppError::Database`] if the database read fails.
pub async fn get_character(
    State(state): State<Arc<AppState>>,
    Path(character_id): Path<String>,
) -> Result<(StatusCode, Json<Character>), AppError> {
    let character = state.characters.get_character(character_id.clone()).await?
        .ok_or_else(|| AppError::NotFound(format!("Character with id, {}, Does not exist.", character_id)))?;
    Ok((StatusCode::OK, Json(character)))
}

/// `POST /character/update` — replace an existing character with the provided
/// data (matched by ID).
///
/// # Errors
///
/// - [`AppError::Database`] if the database write fails.
pub async fn update_character(
    State(state): State<Arc<AppState>>,
    Json(updated_character): Json<Character>,
) -> Result<StatusCode, AppError> {
    state.characters.update_character(&updated_character).await?;
    Ok(StatusCode::OK)
}

/// `POST /character/new` — create a new character and register it on the
/// owning user's character list.
///
/// # Errors
///
/// - [`AppError::NotFound`] if the specified `player_id` does not correspond
///   to an existing user.
/// - [`AppError::Database`] if any database read or write fails.
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
