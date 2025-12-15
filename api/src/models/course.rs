//! Course model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Course {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub price_cents: i32,
    pub is_published: bool,
    pub instructor_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Lesson {
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub position: i32,
    pub is_free: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCourse {
    pub title: String,
    pub description: Option<String>,
    pub price_cents: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourse {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price_cents: Option<i32>,
    pub is_published: Option<bool>,
}
