//! game.rs
//!
//! Route handlers for games.
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

// GET /game/:game_id -> Gets the game from the DB based on the game id
pub async fn get_game(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<String>,
) -> Result<(StatusCode, Json<Game>), AppError> {
    Err(AppError::NotImplemented)
}

// GET /games -> Retrieves all the games from the DB
pub async fn get_all_games(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<Game>>), AppError> {
    Err(AppError::NotImplemented)
}

// POST /game/:game_id -> Updates a game in the DB
pub async fn update_game(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<String>,
    Json(updated_game): Json<Game>,
) -> Result<StatusCode, AppError> {
    Err(AppError::NotImplemented)
}

// POST /game/new -> Creates a new game in the DB
pub async fn create_game(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateGameRequest>,
) -> Result<StatusCode, AppError> {
    Err(AppError::NotImplemented)
}
