use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: Replace these placeholder fields with your actual domain model.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    /// Flexible JSON blob — swap this out for typed fields once the schema is known.
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub data: Option<serde_json::Value>,
}

impl Item {
    pub fn new(req: CreateItemRequest) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: req.name,
            data: req.data,
        }
    }
}
