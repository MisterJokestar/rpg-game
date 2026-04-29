use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_firestore_id")]
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub secret: String,
    #[serde(default)]
    pub characters: Vec<String>
}

#[derive(Deserialize)]
pub struct UsernameOnly {
    pub username: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserRequest {

}

#[derive(Debug, Clone, Deserialize)]
pub struct LogInRequest {

}

#[derive(Debug, Clone, Serialize)]
pub struct LogInResponse {

}
