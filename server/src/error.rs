//! Application-level error type.
//!
//! [`AppError`] is the single error type returned by all route handlers. It
//! implements [`axum::response::IntoResponse`] so Axum can convert it
//! directly into an HTTP response with an appropriate status code and a JSON
//! body of the form `{ "error": "<message>" }`.
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Typed application errors returned by route handlers and middleware.
///
/// Each variant maps to a specific HTTP status code when converted to a
/// response via [`IntoResponse`].
#[derive(Debug, Error)]
pub enum AppError {
    /// A requested resource could not be found. Maps to `404 Not Found`.
    #[error("Not found: {0}")]
    NotFound(String),

    /// A database operation failed. The inner message is logged server-side;
    /// the client receives a generic `"A database error occurred"` message.
    /// Maps to `500 Internal Server Error`.
    #[error("Database error: {0}")]
    Database(String),

    /// The request lacked valid credentials. Maps to `401 Unauthorized`.
    #[allow(dead_code)]
    #[error("Unauthorized")]
    Unauthorized,

    /// The request payload was malformed or violated a business rule. Maps to
    /// `400 Bad Request`.
    #[allow(dead_code)]
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// An unexpected server-side failure occurred. The inner message is logged
    /// server-side; the client receives a generic message. Maps to
    /// `500 Internal Server Error`.
    #[allow(dead_code)]
    #[error("Internal server error: {0}")]
    Internal(String),

    /// The endpoint exists but has not been implemented yet. Maps to
    /// `501 Not Implemented`.
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
