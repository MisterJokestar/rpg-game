use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bcrypt::{hash, verify};
use rand::RngExt;

use crate::error::AppError;

const HASH_COST: u32 = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[cfg_attr(feature = "firestore", serde(rename = "_firestore_id"))]
    #[cfg_attr(feature = "mongodb", serde(rename = "_id"))]
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub secret: String,
    #[serde(default)]
    pub characters: Vec<String>
}

impl User {
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

    pub fn generate_secret(&mut self) {
        let mut secret_bytes = [0u8; 32];
        rand::rng().fill(&mut secret_bytes);
        let secret = secret_bytes.iter().map(|b| format!("{:02x}", b)).collect();
        self.secret = secret;
    }

    pub fn check_password(&self, password: String) -> Result<bool, AppError> {
        verify(password, &self.password_hash)
            .map_err(|e| AppError::Internal(e.to_string()))
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct UsernameOnly {
    pub username: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogInRequest {
    pub username: String,
    pub password: String

}

#[derive(Debug, Clone, Serialize)]
pub struct LogInResponse {
    pub user_id: String,
    pub secret: String
}
