use axum::{extract::Request, middleware::Next, response::Response};

/// Authentication middleware.
///
/// TODO: Replace the pass-through below with real auth logic.
///       Suggested approach:
///         1. Extract the Secrets and username.
///         2. Validate secrets.
///         3. Return `StatusCode::UNAUTHORIZED` (and `AppError::Unauthorized`)
///            for any request that fails validation.
///         4. Pass along validated request.
pub async fn auth_middleware(request: Request, next: Next) -> Response {
    // TODO: implement authentication — currently all requests are allowed through.
    next.run(request).await
}
