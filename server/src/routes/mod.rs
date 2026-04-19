use axum::{middleware, Router, routing::{post, put, get}};
use std::sync::Arc;

use crate::{middleware::auth::auth_middleware, state::AppState};
use crate::routes::character::{create_character, update_character};
use crate::routes::game::{send_action, start_game, stop_game};
//use crate::routes::game::start_game;
// TODO: Replace with game/user routes.
// use self::items::{create_item, delete_item, get_item, list_items, update_item};

mod items;
mod character;
mod state;
mod game;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // TODO: Add game/user routes here.
        // .route("/items", get(list_items).post(create_item))
        // .route("/items/:id", get(get_item).put(update_item).delete(delete_item))
        .route("/games", post(start_game))
        .route("/games/:user_id/stop", post(stop_game))
        .route("/games/:user_id/actions", post(send_action))
        //.route("/games/:user_id/stream", get(game_stream))
        .route("/characters", post(create_character))
        .route("/characters/:id", put(update_character))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
