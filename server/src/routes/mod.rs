use axum::{middleware, routing::get, Router};
use std::sync::Arc;

use crate::{middleware::auth::auth_middleware, state::AppState};

use self::items::{create_item, delete_item, get_item, list_items, update_item};

mod items;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Item CRUD
        .route("/items", get(list_items).post(create_item))
        .route("/items/:id", get(get_item).put(update_item).delete(delete_item))
        // Auth middleware wraps every route above this point.
        // Add new route groups before this layer so they are also protected.
        .layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
