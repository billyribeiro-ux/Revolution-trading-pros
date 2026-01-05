//! Weekly Watchlist Models - Revolution Trading Pros
//! ═══════════════════════════════════════════════════════════════════════════
//! Apple ICT 11+ Principal Engineer Grade - January 2026
//!
//! Automated Weekly Watchlist system with date switcher support

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ═══════════════════════════════════════════════════════════════════════════
// MAIN WATCHLIST MODEL
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WatchlistEntry {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub trader: String,
    pub trader_image: Option<String>,
    pub date_posted: String,
    pub week_of: NaiveDate,
    pub video_url: String,
    pub video_poster: Option<String>,
    pub video_title: String,
    pub spreadsheet_url: String,
    pub watchlist_dates: Option<serde_json::Value>,
    pub description: Option<String>,
    pub status: String,
    pub rooms: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// ═══════════════════════════════════════════════════════════════════════════
// REQUEST DTOs
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct CreateWatchlistRequest {
    pub title: String,
    pub trader: String,
    #[serde(rename = "traderImage")]
    pub trader_image: Option<String>,
    #[serde(rename = "weekOf")]
    pub week_of: String, // Will be parsed to NaiveDate
    pub slug: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "videoSrc")]
    pub video_src: Option<String>,
    #[serde(rename = "videoPoster")]
    pub video_poster: Option<String>,
    #[serde(rename = "spreadsheetSrc")]
    pub spreadsheet_src: Option<String>,
    #[serde(rename = "watchlistDates")]
    pub watchlist_dates: Option<Vec<WatchlistDate>>,
    pub status: Option<String>,
    pub rooms: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWatchlistRequest {
    pub title: Option<String>,
    pub trader: Option<String>,
    #[serde(rename = "traderImage")]
    pub trader_image: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "videoSrc")]
    pub video_src: Option<String>,
    #[serde(rename = "videoPoster")]
    pub video_poster: Option<String>,
    #[serde(rename = "spreadsheetSrc")]
    pub spreadsheet_src: Option<String>,
    #[serde(rename = "watchlistDates")]
    pub watchlist_dates: Option<Vec<WatchlistDate>>,
    pub status: Option<String>,
    pub rooms: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WatchlistDate {
    pub date: String,
    #[serde(rename = "spreadsheetUrl")]
    pub spreadsheet_url: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// RESPONSE DTOs
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct WatchlistResponse {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub subtitle: String,
    pub trader: String,
    #[serde(rename = "traderImage", skip_serializing_if = "Option::is_none")]
    pub trader_image: Option<String>,
    #[serde(rename = "datePosted")]
    pub date_posted: String,
    #[serde(rename = "weekOf")]
    pub week_of: String,
    pub video: VideoData,
    pub spreadsheet: SpreadsheetData,
    #[serde(rename = "watchlistDates", skip_serializing_if = "Option::is_none")]
    pub watchlist_dates: Option<Vec<WatchlistDate>>,
    pub description: String,
    pub status: String,
    pub rooms: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<NavigationLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<NavigationLink>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct VideoData {
    pub src: String,
    pub poster: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct SpreadsheetData {
    pub src: String,
}

#[derive(Debug, Serialize)]
pub struct NavigationLink {
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct WatchlistListResponse {
    pub success: bool,
    pub data: Vec<WatchlistResponse>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct WatchlistSingleResponse {
    pub success: bool,
    pub data: WatchlistResponse,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub total: i64,
    pub last_page: i64,
}

// ═══════════════════════════════════════════════════════════════════════════
// CONVERSION HELPERS
// ═══════════════════════════════════════════════════════════════════════════

impl WatchlistEntry {
    pub fn to_response(&self) -> WatchlistResponse {
        let rooms: Vec<String> = self.rooms
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let watchlist_dates: Option<Vec<WatchlistDate>> = self.watchlist_dates
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok());

        WatchlistResponse {
            id: self.id,
            slug: self.slug.clone(),
            title: self.title.clone(),
            subtitle: self.subtitle.clone().unwrap_or_default(),
            trader: self.trader.clone(),
            trader_image: self.trader_image.clone(),
            date_posted: self.date_posted.clone(),
            week_of: self.week_of.to_string(),
            video: VideoData {
                src: self.video_url.clone(),
                poster: self.video_poster.clone().unwrap_or_default(),
                title: self.video_title.clone(),
            },
            spreadsheet: SpreadsheetData {
                src: self.spreadsheet_url.clone(),
            },
            watchlist_dates,
            description: self.description.clone().unwrap_or_default(),
            status: self.status.clone(),
            rooms,
            previous: None,
            next: None,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
