use axum::{middleware, Router, routing::{post, put, get}};
use std::sync::Arc;

use crate::{middleware::auth::auth_middleware, routes::game::{game_stream, get_game}, state::AppState};
use crate::routes::character::{create_character, update_character};
use crate::routes::game::{send_action, start_game, stop_game};

mod character;
mod game;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/get_game", post(get_game))
        .route("/games", post(start_game))
        .route("/games/:user_id/stop", post(stop_game))
        .route("/games/:user_id/actions", post(send_action))
        .route("/games/:user_id/stream", get(game_stream))
        .route("/characters", post(create_character))
        .route("/characters/:id", put(update_character))
        //.layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
