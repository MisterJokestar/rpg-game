use axum::{Router, http::{Method, header::{AUTHORIZATION, CONTENT_TYPE}}, middleware, routing::{get, post}};
use tower_http::cors::{CorsLayer, Any};
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
    let authed = Router::new()
        .route("/games/:game_id", post(start_game))
        .route("/games/:game_id/stop", post(stop_game))
        .route("/games/:game_id/actions", post(send_action))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    let public = Router::new()
        .route("/games/:game_id/stream", get(game_stream));

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
