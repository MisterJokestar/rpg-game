//! Game record route handlers.
//!
//! Provides CRUD operations for persisted game records. Read operations are
//! public; write operations require authentication.
//!
//! - `GET  /game/:game_id` — fetch a game by ID (public).
//! - `GET  /games`          — list all games (public).
//! - `POST /game/:game_id`  — replace an existing game record (authenticated).
//! - `POST /game/new`       — create a new game (authenticated).
use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json
};
use crate::{
    error::AppError,
    models::game::{CreateGameRequest, Game},
    state::AppState
};

/// `GET /game/:game_id` — retrieve a game record by its unique ID.
///
/// # Errors
///
/// - [`AppError::NotFound`] if no game exists with the given ID.
/// - [`AppError::Database`] if the database read fails.
pub async fn get_game(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<String>,
) -> Result<(StatusCode, Json<Game>), AppError> {
    let game = state.games.get_game(game_id.clone()).await?
        .ok_or_else(|| AppError::NotFound(format!("Game with id, {}, Does not exist.", game_id)))?;
    Ok((StatusCode::OK, Json(game)))
}

/// `GET /games` — retrieve all game records.
///
/// # Errors
///
/// - [`AppError::Database`] if the database read fails.
pub async fn get_all_games(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<Game>>), AppError> {
    let games = state.games.get_all_games().await?;
    Ok((StatusCode::OK, Json(games)))
}

/// `POST /game/:game_id` — replace a game record with the provided body.
///
/// # Errors
///
/// - [`AppError::Database`] if the database write fails.
pub async fn update_game(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<String>,
    Json(updated_game): Json<Game>,
) -> Result<StatusCode, AppError> {
    state.games.update_game(game_id, &updated_game).await?;
    Ok(StatusCode::OK)
}

/// `POST /game/new` — create a new game record and link it to the player's
/// character.
///
/// Creates the game, fetches the character to verify it exists, and appends
/// the new game ID to the character's game list.
///
/// # Errors
///
/// - [`AppError::NotFound`] if the specified `character_id` does not exist.
/// - [`AppError::Database`] if any database read or write fails.
pub async fn create_game(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateGameRequest>,
) -> Result<StatusCode, AppError> {
    let new_game = Game::new(request.player_id, request.character_id.clone());
    let mut character = state.characters.get_character(request.character_id.clone()).await?
            .ok_or_else(|| AppError::NotFound(format!("Character Not Found with id {}", request.character_id)))?;
    state.games.create_game(&new_game).await?;
    character.games.push(new_game.id.clone());
    state.characters.update_character(&character).await?;
    Ok(StatusCode::CREATED)
}
