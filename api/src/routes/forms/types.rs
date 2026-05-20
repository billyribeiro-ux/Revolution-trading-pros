//! Shared DTOs for forms module.
//!
//! Pure data types: row structs (with sqlx::FromRow) and request/query
//! deserializers. Stays `pub` so external consumers (tests, route
//! re-exports) keep the same import paths after the split.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FormRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub fields: serde_json::Value,
    pub settings: serde_json::Value,
    pub is_published: bool,
    pub submission_count: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FormSubmissionRow {
    pub id: i64,
    pub form_id: i64,
    pub data: serde_json::Value,
    pub status: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct FormListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFormRequest {
    pub name: String,
    pub description: Option<String>,
    pub fields: serde_json::Value,
    pub settings: Option<serde_json::Value>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFormRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub fields: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SubmissionListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubmissionStatusRequest {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct BulkUpdateStatusRequest {
    pub submission_ids: Vec<i64>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct BulkDeleteRequest {
    pub submission_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>,
}
