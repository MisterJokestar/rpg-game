//! Authentication middleware.
//!
//! [`auth_middleware`] is applied to all protected routes. It validates the
//! `Authorization` header and rejects requests that lack valid credentials
//! before they reach the route handler.
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response
};
use std::sync::Arc;

use crate::{
    error::AppError,
    state::AppState
};

/// Verify that the incoming request carries a valid `Authorization` header.
///
/// The expected format is:
///
/// ```text
/// Authorization: <user_id>:<secret>
/// ```
///
/// Where `<user_id>` is the UUID of the authenticated user and `<secret>` is
/// the session secret returned by `/login` or `/create_user`.
///
/// # Errors
///
/// - [`AppError::Unauthorized`] if the header is missing, malformed, or the
///   secret does not match the stored value.
/// - [`AppError::NotFound`] if no user exists with the supplied ID.
/// - [`AppError::Database`] if the secret lookup fails.
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
    // parse auth <user_id>:<secret>
    let mut parts = auth.splitn(2, ":");
    let id: &str = parts.next().ok_or(AppError::Unauthorized)?;
    let secret: &str = parts.next().ok_or(AppError::Unauthorized)?;

    // Find the user by ID from the DB, returns 401 if not found
    let db_secret = state.users.get_secret_for_user(String::from(id)).await?
        .ok_or_else(|| AppError::NotFound(format!("Username '{}' not found", id)))?;

    // Validate the secret matches; reject with 401 if it doesn't
    if db_secret != secret { return Err(AppError::Unauthorized);}

    Ok(next.run(request).await)
}
