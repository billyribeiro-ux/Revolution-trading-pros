//! Video Model
//!
//! ICT 11+ Principal Engineer Grade
//! Database model for room_daily_videos table with navigation support

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Room Daily Video - Main video entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RoomDailyVideo {
    pub id: Uuid,
    pub trading_room_id: Uuid,
    pub trader_id: Option<Uuid>,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub duration: Option<i32>,
    pub is_published: bool,
    pub is_featured: bool,
    pub views_count: i32,
    pub published_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Trader info for video response
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RoomTrader {
    pub id: Uuid,
    pub trading_room_id: Uuid,
    pub name: String,
    pub slug: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Trading room info
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradingRoom {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub external_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Video with trader info - for list responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoWithTrader {
    pub id: Uuid,
    pub trading_room_id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub duration: Option<i32>,
    pub is_published: bool,
    pub is_featured: bool,
    pub views_count: i32,
    pub published_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub trader: Option<TraderInfo>,
}

/// Minimal trader info for embedding in video responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraderInfo {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub avatar_url: Option<String>,
}

/// Navigation reference for Previous/Next
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoNavReference {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
}

/// Video with navigation - for single video detail page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoWithNavigation {
    pub video: VideoWithTrader,
    pub previous_video: Option<VideoNavReference>,
    pub next_video: Option<VideoNavReference>,
}

/// Request to create a new video
#[derive(Debug, Clone, Deserialize)]
pub struct CreateVideo {
    pub trading_room_id: Uuid,
    pub trader_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub duration: Option<i32>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Request to update a video
#[derive(Debug, Clone, Deserialize, Default)]
pub struct UpdateVideo {
    pub trader_id: Option<Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration: Option<i32>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Query parameters for listing videos
#[derive(Debug, Clone, Deserialize, Default)]
pub struct VideoListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub room_slug: Option<String>,
    pub trader_id: Option<Uuid>,
    pub search: Option<String>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
}

impl VideoListQuery {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> i64 {
        self.per_page.unwrap_or(12).clamp(1, 100)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.per_page()
    }
}
