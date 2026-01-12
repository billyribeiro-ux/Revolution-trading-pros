//! Page Layout Models for Course Page Builder
//! Apple Principal Engineer ICT 7 Grade - January 2026

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Page Layout - Stores the block configuration for a course page
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PageLayout {
    pub id: Uuid,
    pub course_id: Option<Uuid>,
    pub title: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub version: Option<i32>,
    pub blocks: serde_json::Value,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

/// Simplified layout for list views
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PageLayoutListItem {
    pub id: Uuid,
    pub course_id: Option<Uuid>,
    pub title: String,
    pub slug: Option<String>,
    pub status: Option<String>,
    pub version: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Page Layout Version - For history/undo
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PageLayoutVersion {
    pub id: Uuid,
    pub layout_id: Uuid,
    pub version: i32,
    pub blocks: serde_json::Value,
    pub created_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
}

/// Request to create a new page layout
#[derive(Debug, Deserialize)]
pub struct CreatePageLayoutRequest {
    pub course_id: Option<Uuid>,
    pub title: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub blocks: Option<serde_json::Value>,
}

/// Request to update an existing page layout
#[derive(Debug, Deserialize)]
pub struct UpdatePageLayoutRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub blocks: Option<serde_json::Value>,
}

/// Query parameters for listing layouts
#[derive(Debug, Deserialize)]
pub struct PageLayoutQueryParams {
    pub course_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Paginated response for layouts
#[derive(Debug, Serialize)]
pub struct PaginatedLayouts {
    pub layouts: Vec<PageLayoutListItem>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}
