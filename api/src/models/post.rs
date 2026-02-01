//! Post/Blog models - Revolution Trading Pros
//! ICT Level 7 Principal Engineer Grade - February 2026
//!
//! Blog post models with full SEO support, draft/publish workflow,
//! category hierarchy, and tag relationships.

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
    pub category_id: Option<i64>,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub featured_image_alt: Option<String>,
    pub status: String,
    pub published_at: Option<NaiveDateTime>,
    // SEO fields (ICT 7: comprehensive SEO metadata)
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: bool,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub twitter_card: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Post response with author info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostResponse {
    pub id: i64,
    pub author_id: i64,
    pub author_name: Option<String>,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub featured_image_alt: Option<String>,
    pub status: String,
    pub published_at: Option<NaiveDateTime>,
    // SEO fields
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: bool,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub twitter_card: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // Related data
    pub tags: Option<Vec<TagSimple>>,
}

/// Simple tag for embedding in responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagSimple {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

/// Create post request
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePost {
    #[validate(length(min = 1, max = 255, message = "Title must be 1-255 characters"))]
    pub title: String,
    pub category_id: Option<i64>,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub featured_image_alt: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<NaiveDateTime>,
    // SEO fields
    pub meta_title: Option<String>,
    #[validate(length(max = 160, message = "Meta description must be 160 characters or less"))]
    pub meta_description: Option<String>,
    pub indexable: Option<bool>,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub twitter_card: Option<String>,
    // Tags to associate
    pub tag_ids: Option<Vec<i64>>,
}

/// Update post request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePost {
    #[validate(length(min = 1, max = 255, message = "Title must be 1-255 characters"))]
    pub title: Option<String>,
    pub category_id: Option<i64>,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub featured_image_alt: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<NaiveDateTime>,
    // SEO fields
    pub meta_title: Option<String>,
    #[validate(length(max = 160, message = "Meta description must be 160 characters or less"))]
    pub meta_description: Option<String>,
    pub indexable: Option<bool>,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub twitter_card: Option<String>,
    // Tags to associate (replaces existing)
    pub tag_ids: Option<Vec<i64>>,
}

/// Post list query parameters
#[derive(Debug, Deserialize)]
pub struct PostListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub author_id: Option<i64>,
    pub category_id: Option<i64>,
    pub tag_id: Option<i64>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

/// Post-Tag relationship for many-to-many
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PostTag {
    pub post_id: i64,
    pub tag_id: i64,
}
