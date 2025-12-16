//! Post/Blog model and related types

use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;
use crate::error::ApiError;

/// Deserialize DateTime from Postgres timestamp format
fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s: String = String::deserialize(deserializer)?;
    
    // Try parsing various formats
    if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
        return Ok(dt.with_timezone(&Utc));
    }
    
    // Postgres format: "2025-12-06 13:37:36+00"
    if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%#z") {
        return Ok(dt.with_timezone(&Utc));
    }
    
    // Without timezone
    if let Ok(naive) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
        return Ok(DateTime::from_naive_utc_and_offset(naive, Utc));
    }
    
    Err(Error::custom(format!("Cannot parse datetime: {}", s)))
}

/// Deserialize optional DateTime
fn deserialize_option_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
                return Ok(Some(dt.with_timezone(&Utc)));
            }
            if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%#z") {
                return Ok(Some(dt.with_timezone(&Utc)));
            }
            if let Ok(naive) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
                return Ok(Some(DateTime::from_naive_utc_and_offset(naive, Utc)));
            }
            Err(Error::custom(format!("Cannot parse datetime: {}", s)))
        }
    }
}

/// Deserialize i64 from either number or string (Neon returns BIGINT as string)
fn deserialize_i64_from_string<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i64),
    }
    
    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<i64>().map_err(Error::custom),
        StringOrInt::Int(i) => Ok(i),
    }
}

/// Blog post entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub featured_image: Option<String>,
    pub author_id: Uuid,
    pub category_id: Option<Uuid>,
    pub status: PostStatus,
    pub visibility: PostVisibility,
    #[serde(default, deserialize_with = "deserialize_option_datetime")]
    pub published_at: Option<DateTime<Utc>>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub reading_time: Option<i32>,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub view_count: i64,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostStatus {
    Draft,
    Published,
    Scheduled,
    Archived,
}

impl Default for PostStatus {
    fn default() -> Self {
        Self::Draft
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostVisibility {
    Public,
    Private,
    Members,
}

impl Default for PostVisibility {
    fn default() -> Self {
        Self::Public
    }
}

/// Post with relations (for API responses)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostWithRelations {
    #[serde(flatten)]
    pub post: Post,
    pub author: Option<PostAuthor>,
    pub category: Option<PostCategory>,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

/// Minimal category for post responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCategory {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostAuthor {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

/// Category entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub image_url: Option<String>,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub post_count: i64,
    pub is_visible: bool,
    pub sort_order: i32,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tag entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub post_count: i64,
    pub is_visible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create post request
#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub slug: Option<String>,
    pub excerpt: Option<String>,
    pub content: String,
    pub featured_image: Option<String>,
    pub category_id: Option<Uuid>,
    pub tag_ids: Option<Vec<Uuid>>,
    pub status: Option<PostStatus>,
    pub visibility: Option<PostVisibility>,
    pub published_at: Option<DateTime<Utc>>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

impl CreatePostRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.title.trim().is_empty() {
            return Err(ApiError::Validation("Title is required".to_string()));
        }
        if self.content.trim().is_empty() {
            return Err(ApiError::Validation("Content is required".to_string()));
        }
        Ok(())
    }
}

/// Update post request
#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub featured_image: Option<String>,
    pub category_id: Option<Uuid>,
    pub tag_ids: Option<Vec<Uuid>>,
    pub status: Option<PostStatus>,
    pub visibility: Option<PostVisibility>,
    pub published_at: Option<DateTime<Utc>>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

/// Create category request
#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub image_url: Option<String>,
    pub is_visible: Option<bool>,
    pub sort_order: Option<i32>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

impl CreateCategoryRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.name.trim().is_empty() {
            return Err(ApiError::Validation("Name is required".to_string()));
        }
        Ok(())
    }
}

/// Create tag request
#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub is_visible: Option<bool>,
}

impl CreateTagRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.name.trim().is_empty() {
            return Err(ApiError::Validation("Name is required".to_string()));
        }
        Ok(())
    }
}

/// Post list query parameters
#[derive(Debug, Deserialize)]
pub struct PostListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<PostStatus>,
    pub category_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    pub author_id: Option<Uuid>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl PostListQuery {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> i64 {
        self.per_page.unwrap_or(10).clamp(1, 100)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.per_page()
    }
}

/// Paginated response
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    pub has_more: bool,
}

impl PaginationMeta {
    pub fn new(page: i64, per_page: i64, total: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Self {
            current_page: page,
            per_page,
            total,
            total_pages,
            has_more: page < total_pages,
        }
    }
}

/// Media/Image entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: Uuid,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub blurhash: Option<String>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub uploaded_by: Uuid,
    pub folder: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Video entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub provider: VideoProvider,
    pub provider_id: String,
    pub thumbnail_url: Option<String>,
    pub duration: Option<i32>,
    pub is_public: bool,
    pub view_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoProvider {
    Vimeo,
    Youtube,
    Wistia,
    Custom,
}
