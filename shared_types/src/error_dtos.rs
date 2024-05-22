use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorDto {
    pub id: Uuid,
    pub status_code: i16,
    pub user_affected: String,
    pub path: String,
    pub line: i32,
    pub message: String,
    pub stack_trace: String,
    pub namespace_id: Uuid,
    pub resolved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateErrorDto {
    pub status_code: i16,
    pub user_affected: String,
    pub path: String,
    pub line: i32,
    pub stack_trace: String,
    pub message: String,
    pub namespace_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateErrorDto {
    pub id: Uuid,
    pub resolved: Option<bool>,
}
