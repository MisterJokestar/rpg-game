//! character.rs
//!
//! Routes for character management.
//! Allows users to create and update characters.

use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;
use uuid::Uuid;
use crate::{
    error::AppError,
    models::user::{AddCharacterRequest, Character, UpdateCharacterRequest},
    state::AppState,
};

// Handler for POST: creates a new character and adds it to the users account
pub async fn create_character(
    State(state): State<Arc<AppState>>,
    Json(body): Json<AddCharacterRequest>,
) -> Result<(StatusCode, Json<Character>), AppError> {
    // Find the user by username from the DB, returns 401 if not found
    let user = state.users.get_user_by_username(&body.username).await?
        .ok_or_else(|| AppError::NotFound(format!("Username '{}' not found", &body.username)))?;

    // Validate the secret matches, if not, return 404
    if user.secret != body.secret { return Err(AppError::Unauthorized);}
    // Call add_character handler and pass JSON body to create the new character
    let updated = user.add_character(body);
    // Making copy of the new character
    let new_character = updated.characters.last().cloned().unwrap();
    state.users.update_user(updated).await?;
    // Return 201 with the new character as JSON
    Ok((StatusCode::CREATED, Json(new_character)))

}

// Handler for PUT: update an existing character in the users account
pub async fn update_character(
    State(state): State<Arc<AppState>>,
    Path(character_id): Path<Uuid>,
    Json(body): Json<UpdateCharacterRequest>,
) -> Result<Json<Character>, AppError> {
    // Find the user by username from the DB, returns a 401 if not found
    let user = state.users.get_user_by_username(&body.username).await?
        .ok_or_else(|| AppError::NotFound(format!("Username '{}' not found", &body.username)))?;
    // Validate the secret matches, if not, return 404
    if user.secret != body.secret { return Err(AppError::Unauthorized);}
    // Apply updates to the character, returns a 404 if character_id is not found
    let updated = user.update_character(character_id, body)
        .ok_or_else(||AppError::NotFound(format!("Character '{}' not found", character_id)))?;
    // Find the updated character to return in the response
    let character = updated.characters.iter().find(|c| c.id == character_id).cloned().unwrap();

    state.users.update_user(updated).await?;
    Ok(Json(character))
}