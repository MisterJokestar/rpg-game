use std::sync::Arc;

use crate::db::Repository;

/// Shared application state injected into every route handler via axum's
/// `State` extractor. Wrap new shared resources (config, cache clients, etc.)
/// in this struct rather than reaching for globals.
pub struct AppState {
    pub db: Arc<dyn Repository>,
}
