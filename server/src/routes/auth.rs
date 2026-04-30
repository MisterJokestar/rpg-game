//! auth.rs
//!
//! Route handlers for logging in/ creating a new account.
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

// POST /create_user -> Creates a new user account
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

// POST /login -> Logs in to a users account
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
