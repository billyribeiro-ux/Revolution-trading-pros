//! Shared types for the `trading_rooms` route module.
//!
//! R27-B5 split: moved verbatim from the pre-split `trading_rooms.rs`.
//! These DTOs are part of the public API: `tests/trading_rooms_test.rs`
//! pins their wire shapes (`TradingRoom.type` JSON key, FromRow round-
//! trip, optional query DTO accept-empty-payload behavior). Do NOT
//! rename fields, drop derives, or move sqlx renames without updating
//! the matching test.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradingRoom {
    pub id: i64,
    pub name: String,
    pub slug: String,
    #[sqlx(rename = "type")]
    pub room_type: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub logo_url: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    pub is_featured: bool,
    pub is_public: bool,
    pub available_sections: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RoomSection {
    pub id: i64,
    pub section_key: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    pub allowed_resource_types: Option<serde_json::Value>,
    pub max_items: Option<i32>,
    pub restricted_to_rooms: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct TradingRoomsQuery {
    pub with_counts: Option<bool>,
    pub active_only: Option<bool>,
    pub room_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TradersQuery {
    pub room_slug: Option<String>,
    pub active_only: Option<bool>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct VideosQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub room_slug: Option<String>,
    pub content_type: Option<String>,
    pub is_published: Option<bool>,
}
