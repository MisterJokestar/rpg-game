use axum::{middleware, Router};
use std::sync::Arc;

use crate::{middleware::auth::auth_middleware, state::AppState};

// TODO: Replace with game/user routes.
// use self::items::{create_item, delete_item, get_item, list_items, update_item};

mod items;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // TODO: Add game/user routes here.
        // .route("/items", get(list_items).post(create_item))
        // .route("/items/:id", get(get_item).put(update_item).delete(delete_item))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
