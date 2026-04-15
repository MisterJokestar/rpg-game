use axum::{extract::Request, middleware::Next, response::Response};

/// Authentication middleware.
///
/// TODO: Replace the pass-through below with real auth logic.
///       Suggested approach:
///         1. Extract the `Authorization: Bearer <token>` header.
///         2. Validate the JWT (signature, expiry, issuer).
///         3. Inject the verified claims into request extensions so handlers
///            can access the authenticated user identity.
///         4. Return `StatusCode::UNAUTHORIZED` (and `AppError::Unauthorized`)
///            for any request that fails validation.
pub async fn auth_middleware(request: Request, next: Next) -> Response {
    // TODO: implement authentication — currently all requests are allowed through.
    next.run(request).await
}
