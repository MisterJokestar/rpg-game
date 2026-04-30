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
    let game = state.games.get_game(game_id.clone()).await?
        .ok_or_else(|| AppError::NotFound(format!("Game with id, {}, Does not exist.", game_id)))?;
    Ok((StatusCode::OK, Json(game)))
}

// GET /games -> Retrieves all the games from the DB
pub async fn get_all_games(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<Game>>), AppError> {
    let games = state.games.get_all_games().await?;
    Ok((StatusCode::OK, Json(games)))
}

// POST /game/:game_id -> Updates a game in the DB
pub async fn update_game(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<String>,
    Json(updated_game): Json<Game>,
) -> Result<StatusCode, AppError> {
    state.games.update_game(game_id, &updated_game).await?;
    Ok(StatusCode::OK)
}

// POST /game/new -> Creates a new game in the DB
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
