//! User account model.
//!
//! Provides [`User`] for persistence, [`CreateUserRequest`] / [`LogInRequest`]
//! for incoming JSON payloads, and [`LogInResponse`] for the authentication
//! token returned to the client.
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bcrypt::{hash, verify};
use rand::RngExt;

use crate::error::AppError;

const HASH_COST: u32 = 10;

/// A registered user account stored in the database.
///
/// The `id` field is serialised as `_firestore_id` or `_id` depending on the
/// active database backend feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier (UUID v7).
    #[cfg_attr(feature = "firestore", serde(rename = "_firestore_id"))]
    #[cfg_attr(feature = "mongodb", serde(rename = "_id"))]
    pub id: String,
    /// Human-readable username chosen at registration.
    pub username: String,
    /// bcrypt hash of the user's password.
    pub password_hash: String,
    /// Randomly-generated session secret. Rotated on every successful login.
    /// Clients include this value in the `Authorization` header as
    /// `<user_id>:<secret>`.
    pub secret: String,
    /// IDs of all characters owned by this user.
    #[serde(default)]
    pub characters: Vec<String>
}

impl User {
    /// Create a new user, hashing the provided password with bcrypt.
    ///
    /// A fresh random 32-byte session secret is generated at creation time.
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Internal`] if bcrypt fails to hash the password.
    pub fn new(username: String, password: String) -> Result<Self, AppError> {
        let password_hash = hash(password, HASH_COST)
            .map_err(|_| AppError::Internal("ERROR HASHING PASSWORD".to_string()))?;
        let mut secret_bytes = [0u8; 32];
        rand::rng().fill(&mut secret_bytes);
        let secret = secret_bytes.iter().map(|b| format!("{:02x}", b)).collect();
        Ok(User {
            id: Uuid::now_v7().to_string(),
            username,
            password_hash,
            secret,
            characters: Vec::new()
        })
    }

    /// Rotate the session secret, invalidating any existing client tokens.
    ///
    /// Called after a successful login so that each session gets a unique
    /// secret.
    pub fn generate_secret(&mut self) {
        let mut secret_bytes = [0u8; 32];
        rand::rng().fill(&mut secret_bytes);
        let secret = secret_bytes.iter().map(|b| format!("{:02x}", b)).collect();
        self.secret = secret;
    }

    /// Verify a plain-text password against the stored bcrypt hash.
    ///
    /// Returns `Ok(true)` if the password matches, `Ok(false)` otherwise.
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Internal`] if bcrypt encounters an unexpected error
    /// during verification.
    pub fn check_password(&self, password: String) -> Result<bool, AppError> {
        verify(password, &self.password_hash)
            .map_err(|e| AppError::Internal(e.to_string()))
    }
}

/// Helper projection used by Firestore queries that only need the username.
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct UsernameOnly {
    pub username: String
}

/// Request body for the `POST /create_user` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserRequest {
    /// Desired username. Must be unique across all registered accounts.
    pub username: String,
    /// Plain-text password. Hashed server-side before storage.
    pub password: String
}

/// Request body for the `POST /login` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct LogInRequest {
    /// Username of the account to authenticate.
    pub username: String,
    /// Plain-text password to verify.
    pub password: String

}

/// Response body returned after a successful login or account creation.
///
/// Clients should store both values and pass them as
/// `Authorization: <user_id>:<secret>` on authenticated requests.
#[derive(Debug, Clone, Serialize)]
pub struct LogInResponse {
    /// The authenticated user's unique ID.
    pub user_id: String,
    /// Freshly-generated session secret.
    pub secret: String
}
