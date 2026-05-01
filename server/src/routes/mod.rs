//! HTTP router construction.
//!
//! [`create_router`] assembles the full Axum [`Router`], splitting routes into
//! two groups:
//!
//! - **Public** – no authentication required (`/create_user`, `/login`,
//!   `GET /game/:id`, `GET /games`, `GET /session/:id/stream`).
//! - **Authenticated** – all mutating routes; the [`auth_middleware`] checks
//!   the `Authorization: <user_id>:<secret>` header before the handler runs.
//!
//! CORS is enabled for all origins so that the frontend can reach the API from
//! any host.
use axum::{
    Router,
    http::{Method, header::{AUTHORIZATION, CONTENT_TYPE}},
    middleware,
    routing::{get, post}};
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;

use crate::{
    middleware::auth::auth_middleware,
    routes::{
        session::{
            game_stream,
            send_action,
            start_game,
            stop_game,
        },
        game::{
            get_game,
            get_all_games,
            update_game,
            create_game,
        },
        auth::{
            login,
            create_user,
        },
        character::{
            get_character,
            update_character,
            create_character,
        },
    },
    state::AppState
};

mod session;
mod game;
mod character;
mod auth;

/// Build the application [`Router`] with all routes and middleware applied.
///
/// The router is intended to be passed directly to `axum::serve`.
pub fn create_router(state: Arc<AppState>) -> Router {
    let authed = Router::new()
        .route("/character/:character_id", get(get_character))
        .route("/character/update", post(update_character))
        .route("/character/new", post(create_character))
        .route("/game/:game_id", post(update_game))
        .route("/game/new", post(create_game))
        .route("/session/:game_id", post(start_game))
        .route("/session/:game_id/stop", post(stop_game))
        .route("/session/:game_id/action", post(send_action))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    let public = Router::new()
        .route("/create_user", post(create_user))
        .route("/login", post(login))
        .route("/game/:game_id", get(get_game))
        .route("/games", get(get_all_games))
        .route("/session/:game_id/stream", get(game_stream));

    Router::new()
        .merge(authed)
        .merge(public)
        .with_state(state)
        .layer(
          CorsLayer::new()
              .allow_origin(Any)
              .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
              .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
        )
}
