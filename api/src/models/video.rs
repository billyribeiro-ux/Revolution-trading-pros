//! Unified Video Models - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - January 2026
//!
//! Supports: Daily Videos, Weekly Watchlist, Learning Center, Room Archives
//! Video Platform: Bunny.net (primary)

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ═══════════════════════════════════════════════════════════════════════════
// AVAILABLE TAGS
// ═══════════════════════════════════════════════════════════════════════════

pub const AVAILABLE_TAGS: &[(&str, &str, &str)] = &[
    ("risk-management", "Risk Management", "#ef4444"),
    ("options-strategies", "Options Strategies", "#f59e0b"),
    ("macro-structure", "Macro Structure", "#10b981"),
    ("micro-structure", "Micro Structure", "#06b6d4"),
    ("psychology", "Psychology", "#8b5cf6"),
    ("technical-analysis", "Technical Analysis", "#3b82f6"),
    ("fundamentals", "Fundamentals", "#ec4899"),
    ("trade-setups", "Trade Setups", "#14b8a6"),
    ("market-review", "Market Review", "#6366f1"),
    ("earnings", "Earnings", "#f97316"),
    ("futures", "Futures", "#84cc16"),
    ("forex", "Forex", "#22c55e"),
    ("crypto", "Crypto", "#a855f7"),
    ("small-accounts", "Small Accounts", "#eab308"),
    ("position-sizing", "Position Sizing", "#0ea5e9"),
    ("entry-exit", "Entry & Exit", "#d946ef"),
    ("scanner-setups", "Scanner Setups", "#64748b"),
    ("indicators", "Indicators", "#fb7185"),
];

// ═══════════════════════════════════════════════════════════════════════════
// MAIN VIDEO MODEL
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UnifiedVideo {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub video_url: String,
    pub video_platform: String,
    pub video_id: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub duration: Option<i32>,
    pub quality: Option<String>,
    pub content_type: String,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub session_type: Option<String>,
    pub chapter_timestamps: Option<serde_json::Value>,
    pub trader_id: Option<i64>,
    pub video_date: NaiveDate,
    pub is_published: bool,
    pub is_featured: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub tags: Option<serde_json::Value>,
    pub views_count: i32,
    pub likes_count: i32,
    pub completion_rate: i32,
    pub bunny_library_id: Option<i64>,
    pub bunny_encoding_status: Option<String>,
    pub bunny_thumbnail_url: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoRoomAssignment {
    pub id: i64,
    pub video_id: i64,
    pub trading_room_id: i64,
    pub is_featured_in_room: bool,
    pub is_pinned: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════
// REQUEST DTOs
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct CreateVideoRequest {
    pub title: String,
    pub description: Option<String>,
    pub video_url: String,
    pub video_platform: Option<String>,
    pub content_type: String,
    pub video_date: String,
    pub trader_id: Option<i64>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub room_ids: Option<Vec<i64>>,
    pub upload_to_all: Option<bool>,
    pub thumbnail_url: Option<String>,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub session_type: Option<String>,
    pub duration: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVideoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub video_platform: Option<String>,
    pub content_type: Option<String>,
    pub video_date: Option<String>,
    pub trader_id: Option<i64>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub room_ids: Option<Vec<i64>>,
    pub upload_to_all: Option<bool>,
    pub thumbnail_url: Option<String>,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub session_type: Option<String>,
    pub duration: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct VideoListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub content_type: Option<String>,
    pub room_id: Option<i64>,
    pub room_slug: Option<String>,
    pub trader_id: Option<i64>,
    pub tags: Option<String>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BulkAssignRequest {
    pub video_ids: Vec<i64>,
    pub room_ids: Vec<i64>,
    pub clear_existing: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct BulkPublishRequest {
    pub video_ids: Vec<i64>,
    pub publish: bool,
}

#[derive(Debug, Deserialize)]
pub struct BulkDeleteRequest {
    pub video_ids: Vec<i64>,
    pub force: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════
// RESPONSE DTOs
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct VideoResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub video_url: String,
    pub embed_url: String,
    pub video_platform: String,
    pub thumbnail_url: Option<String>,
    pub duration: Option<i32>,
    pub formatted_duration: String,
    pub content_type: String,
    pub video_date: String,
    pub formatted_date: String,
    pub is_published: bool,
    pub is_featured: bool,
    pub tags: Vec<String>,
    pub tag_details: Vec<TagDetail>,
    pub views_count: i32,
    pub trader: Option<TraderInfo>,
    pub rooms: Vec<RoomInfo>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct TagDetail {
    pub slug: String,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize)]
pub struct TraderInfo {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct RoomInfo {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct VideoListResponse {
    pub success: bool,
    pub data: Vec<VideoResponse>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub total: i64,
    pub last_page: i64,
}

#[derive(Debug, Serialize)]
pub struct VideoStatsResponse {
    pub success: bool,
    pub data: VideoStats,
}

#[derive(Debug, Serialize)]
pub struct VideoStats {
    pub total: i64,
    pub published: i64,
    pub by_type: VideoTypeStats,
    pub total_views: i64,
    pub this_week: i64,
    pub this_month: i64,
}

#[derive(Debug, Serialize)]
pub struct VideoTypeStats {
    pub daily_video: i64,
    pub weekly_watchlist: i64,
    pub learning_center: i64,
    pub room_archive: i64,
}

#[derive(Debug, Serialize)]
pub struct VideoOptionsResponse {
    pub success: bool,
    pub data: VideoOptions,
}

#[derive(Debug, Serialize)]
pub struct VideoOptions {
    pub content_types: Vec<ContentTypeOption>,
    pub platforms: Vec<PlatformOption>,
    pub difficulty_levels: Vec<DifficultyOption>,
    pub tags: Vec<TagDetail>,
    pub trading_rooms: Vec<RoomInfo>,
    pub traders: Vec<TraderInfo>,
}

#[derive(Debug, Serialize)]
pub struct ContentTypeOption {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct PlatformOption {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct DifficultyOption {
    pub value: String,
    pub label: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

impl UnifiedVideo {
    pub fn get_embed_url(&self) -> String {
        match self.video_platform.as_str() {
            "bunny" => {
                if let Some(guid) = &self.bunny_video_guid {
                    if let Some(lib_id) = self.bunny_library_id {
                        return format!("https://iframe.mediadelivery.net/embed/{}/{}", lib_id, guid);
                    }
                }
                self.video_url.clone()
            }
            "vimeo" => {
                if let Some(id) = &self.video_id {
                    format!("https://player.vimeo.com/video/{}", id)
                } else {
                    self.video_url.clone()
                }
            }
            "youtube" => {
                if let Some(id) = &self.video_id {
                    format!("https://www.youtube.com/embed/{}", id)
                } else {
                    self.video_url.clone()
                }
            }
            _ => self.video_url.clone(),
        }
    }

    pub fn get_formatted_duration(&self) -> String {
        match self.duration {
            Some(d) if d > 0 => {
                let hours = d / 3600;
                let minutes = (d % 3600) / 60;
                let seconds = d % 60;
                if hours > 0 {
                    format!("{}:{:02}:{:02}", hours, minutes, seconds)
                } else {
                    format!("{}:{:02}", minutes, seconds)
                }
            }
            _ => String::new(),
        }
    }

    pub fn get_tags_vec(&self) -> Vec<String> {
        self.tags
            .as_ref()
            .and_then(|t| serde_json::from_value::<Vec<String>>(t.clone()).ok())
            .unwrap_or_default()
    }

    pub fn get_tag_details(&self) -> Vec<TagDetail> {
        self.get_tags_vec()
            .iter()
            .filter_map(|slug| {
                AVAILABLE_TAGS.iter().find(|(s, _, _)| s == slug).map(|(s, n, c)| TagDetail {
                    slug: s.to_string(),
                    name: n.to_string(),
                    color: c.to_string(),
                })
            })
            .collect()
    }
}

pub fn get_content_types() -> Vec<ContentTypeOption> {
    vec![
        ContentTypeOption { value: "daily_video".to_string(), label: "Daily Video".to_string() },
        ContentTypeOption { value: "weekly_watchlist".to_string(), label: "Weekly Watchlist".to_string() },
        ContentTypeOption { value: "learning_center".to_string(), label: "Learning Center".to_string() },
        ContentTypeOption { value: "room_archive".to_string(), label: "Room Archive".to_string() },
    ]
}

pub fn get_platforms() -> Vec<PlatformOption> {
    vec![
        PlatformOption { value: "bunny".to_string(), label: "Bunny.net".to_string() },
        PlatformOption { value: "vimeo".to_string(), label: "Vimeo".to_string() },
        PlatformOption { value: "youtube".to_string(), label: "YouTube".to_string() },
        PlatformOption { value: "wistia".to_string(), label: "Wistia".to_string() },
        PlatformOption { value: "direct".to_string(), label: "Direct URL".to_string() },
    ]
}

pub fn get_difficulty_levels() -> Vec<DifficultyOption> {
    vec![
        DifficultyOption { value: "beginner".to_string(), label: "Beginner".to_string() },
        DifficultyOption { value: "intermediate".to_string(), label: "Intermediate".to_string() },
        DifficultyOption { value: "advanced".to_string(), label: "Advanced".to_string() },
    ]
}

pub fn get_all_tags() -> Vec<TagDetail> {
    AVAILABLE_TAGS.iter().map(|(s, n, c)| TagDetail {
        slug: s.to_string(),
        name: n.to_string(),
        color: c.to_string(),
    }).collect()
}
