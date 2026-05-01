//! Axum middleware used by the server.
//!
//! Currently provides [`auth`] for request authentication. Additional
//! middleware (rate-limiting, request tracing, etc.) can be added as further
//! sub-modules here.
pub mod auth;
