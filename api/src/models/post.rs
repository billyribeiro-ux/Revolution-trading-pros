//! Post/Blog models - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// Post status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PostStatus {
    Draft,
    Published,
    Archived,
}

/// Blog post entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub status: String,
    pub published_at: Option<NaiveDateTime>,
    // SEO fields
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: bool,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Post response with author info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostResponse {
    pub id: i64,
    pub author_id: i64,
    pub author_name: Option<String>,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub status: String,
    pub published_at: Option<NaiveDateTime>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: bool,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Create post request
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePost {
    #[validate(length(min = 1, max = 255, message = "Title must be 1-255 characters"))]
    pub title: String,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<NaiveDateTime>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: Option<bool>,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
}

/// Update post request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePost {
    #[validate(length(min = 1, max = 255, message = "Title must be 1-255 characters"))]
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<NaiveDateTime>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: Option<bool>,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
}

/// Post list query parameters
#[derive(Debug, Deserialize)]
pub struct PostListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub author_id: Option<i64>,
    pub search: Option<String>,
}
