//! auth.rs
//!
//! Route handlers for logging in/ creating a new account.
use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    Json
};
use crate::{
    error::AppError,
    models::user::{
        CreateUserRequest,
        LogInRequest,
        LogInResponse
    },
    state::AppState
};

// POST /create_user -> Creates a new user account
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<LogInResponse>), AppError> {
    Err(AppError::NotImplemented)
}

// POST /login -> Logs in to a users account
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LogInRequest>,
) -> Result<(StatusCode, Json<LogInResponse>), AppError> {
    Err(AppError::NotImplemented)
}
