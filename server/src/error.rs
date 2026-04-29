use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(String),

    #[allow(dead_code)]
    #[error("Unauthorized")]
    Unauthorized,

    #[allow(dead_code)]
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[allow(dead_code)]
    #[error("Internal server error: {0}")]
    Internal(String),

    #[allow(dead_code)]
    #[error("Not Implemented")]
    NotImplemented,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "A database error occurred".to_string())
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal error occurred".to_string())
            },
            AppError::NotImplemented => (StatusCode::NOT_IMPLEMENTED, "Not Implemented".to_string())
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
