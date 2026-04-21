use axum::{extract::{Request, State}, middleware::Next, response::Response};
use std::sync::Arc;

use crate::{
    error::AppError,
    state::AppState
};

/// Authentication middleware.
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next
) -> Result<Response, AppError> {
    // Grabs Authorization from Request header
    let auth = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized)?;
    // parse auth <username>:<secret>
    let mut parts = auth.splitn(2, ":");
    let id: &str = parts.next().ok_or(AppError::Unauthorized)?;
    let secret: &str = parts.next().ok_or(AppError::Unauthorized)?;

    // Find the user by username from the DB, returns 401 if not found
    let db_secret = state.users.get_secret_for_user(String::from(id)).await?
        .ok_or_else(|| AppError::NotFound(format!("Username '{}' not found", id)))?;

    // Validate the secret matches, if not, return 404
    if db_secret != String::from(secret) { return Err(AppError::Unauthorized);}
    
    Ok(next.run(request).await)
}
