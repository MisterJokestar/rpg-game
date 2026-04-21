use axum::{middleware, Router, routing::{post, get}};
use std::sync::Arc;

use crate::{
    middleware::auth::auth_middleware, routes::game::{
        game_stream, 
        send_action,
        start_game,
        stop_game,
    }, state::AppState
};

mod game;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/games/:game_id", post(start_game))
        .route("/games/:game_id/stop", post(stop_game))
        .route("/games/:game_id/actions", post(send_action))
        .route("/games/:game_id/stream", get(game_stream))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
