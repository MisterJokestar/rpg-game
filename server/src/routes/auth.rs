//! Authentication route handlers.
//!
//! Exposes two public (unauthenticated) endpoints:
//!
//! - `POST /create_user` — register a new account.
//! - `POST /login` — authenticate and receive a fresh session secret.
use std::sync::Arc;
use axum::{
    Json, extract::State, http::StatusCode
};
use crate::{
    error::AppError,
    models::user::{
        CreateUserRequest,
        LogInRequest,
        LogInResponse, User
    },
    state::AppState
};

/// `POST /create_user` — register a new user account.
///
/// Creates the user record in the database and immediately returns the
/// credentials the client needs for authenticated requests.
///
/// # Errors
///
/// - [`AppError::BadRequest`] if the requested username is already taken.
/// - [`AppError::Internal`] if bcrypt fails to hash the password.
/// - [`AppError::Database`] if the database write fails.
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<LogInResponse>), AppError> {
    let existing_users = state.users.get_users().await?;
    if existing_users.contains(&request.username) {
        Err(AppError::BadRequest(format!("Username, {}, is already taken.", request.username)))
    } else {
        let user = match User::new(request.username, request.password) {
            Ok(u) => u,
            Err(e) => return Err(e),
        };
        state.users.create_user(&user).await?;
        let response = LogInResponse {
            user_id: user.id,
            secret: user.secret,
        };
        Ok((StatusCode::CREATED, Json(response)))
    }
}

/// `POST /login` — authenticate with a username and password.
///
/// On success, rotates the session secret and returns the new credentials.
/// Clients must update their stored secret after each login.
///
/// # Errors
///
/// - [`AppError::NotFound`] if no account exists with the given username.
/// - [`AppError::Unauthorized`] if the password does not match.
/// - [`AppError::Database`] if the database read or write fails.
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LogInRequest>,
) -> Result<(StatusCode, Json<LogInResponse>), AppError> {
    let username = request.username.clone();
    let mut user = state.users.get_user_by_name(request.username).await?
        .ok_or_else(|| AppError::NotFound(format!("User, {}, Does not exist.", username)))?;
    let check = match user.check_password(request.password) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    if check {
        user.generate_secret();
        state.users.update_user(&user).await?;
        let response = LogInResponse {
            user_id: user.id,
            secret: user.secret,
        };
        Ok((StatusCode::OK, Json(response)))
    } else {
        Err(AppError::Unauthorized)
    }
}
