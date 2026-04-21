use axum::{middleware, Router, routing::{post, get}};
use std::sync::Arc;

use crate::{
    routes::game::{
        game_stream, 
        get_game,
        send_action,
        // start_game,
        stop_game,
    },
    state::AppState,
};

mod game;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/get_game", post(get_game))
        // .route("/games", post(start_game))
        .route("/games/:game_id/stop", post(stop_game))
        .route("/games/:game_id/actions", post(send_action))
        .route("/games/:game_id/stream", get(game_stream))
        //.layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
