//! Room Resources API Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Unified resource management for trading rooms supporting:
//! - Videos (Bunny.net, Vimeo, YouTube, Direct)
//! - PDFs (Trade plans, guides)
//! - Documents (Word, Excel)
//! - Images (Charts, screenshots)
//!
//! Features:
//! - Room-specific isolation
//! - Content type filtering
//! - Featured/pinned resources
//! - Presigned URL uploads for R2
//! - Full CRUD operations
//!
//! Split from a single 2,608-LOC `room_resources.rs` file (R5-B
//! maintainability pass, 2026-05-20). Public API unchanged: external
//! callers still see `routes::room_resources::public_router()` and
//! `routes::room_resources::admin_router()` returning `Router<AppState>`.

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

pub mod admin_crud;
pub mod bulk;
pub mod download;
pub mod helpers;
pub mod public;
pub mod versioning;

// ═══════════════════════════════════════════════════════════════════════════
// SHARED TYPES (used by multiple sub-modules)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RoomResource {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub resource_type: String,
    pub content_type: String,
    pub section: Option<String>, // ICT 7: Section field for organization
    pub file_url: String,
    pub file_path: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub video_platform: Option<String>,
    pub video_id: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<i64>,
    pub duration: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub trading_room_id: i64,
    pub trader_id: Option<i64>,
    pub resource_date: NaiveDate,
    pub is_published: bool,
    pub is_featured: bool,
    pub is_pinned: bool,
    pub sort_order: i32,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub scheduled_at: Option<chrono::NaiveDateTime>,
    pub category: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub difficulty_level: Option<String>,
    pub views_count: i32,
    pub downloads_count: i32,
    pub likes_count: i32,
    pub metadata: Option<serde_json::Value>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    // ICT 7 NEW: Access control
    pub access_level: Option<String>, // free, member, premium, vip
    // ICT 7 NEW: Versioning
    pub version: Option<i32>,
    pub previous_version_id: Option<i64>,
    pub is_latest_version: Option<bool>,
    // ICT 7 NEW: Course/Lesson linking
    pub course_id: Option<i64>,
    pub lesson_id: Option<i64>,
    pub course_order: Option<i32>,
    // ICT 7 NEW: Secure download
    pub secure_token: Option<String>,
    pub secure_token_expires: Option<chrono::NaiveDateTime>,
    // ICT 7 NEW: Storage tracking
    pub file_hash: Option<String>,
    pub storage_provider: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResourceListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub room_id: Option<i64>,
    pub room_slug: Option<String>,
    pub resource_type: Option<String>,
    pub content_type: Option<String>,
    pub section: Option<String>, // ICT 7: Filter by section
    pub is_featured: Option<bool>,
    pub is_published: Option<bool>,
    pub tags: Option<String>,
    pub difficulty_level: Option<String>,
    pub search: Option<String>,
    // ICT 7 NEW: Access control and versioning filters
    pub access_level: Option<String>, // free, member, premium, vip
    pub course_id: Option<i64>,       // Filter by course
    pub lesson_id: Option<i64>,       // Filter by lesson
    pub latest_only: Option<bool>,    // Only latest versions (default true)
}

#[derive(Debug, Deserialize)]
pub struct CreateResourceRequest {
    pub title: String,
    pub description: Option<String>,
    pub resource_type: String,
    pub content_type: String,
    pub file_url: String,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub video_platform: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<i64>,
    pub duration: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub trading_room_id: i64,
    pub trader_id: Option<i64>,
    pub resource_date: Option<String>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_pinned: Option<bool>,
    pub section: Option<String>, // ICT 7: Section for organization
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub difficulty_level: Option<String>,
    // ICT 7 NEW: Access control and linking
    pub access_level: Option<String>, // free, member, premium, vip (default: premium)
    pub course_id: Option<i64>,       // Link to course
    pub lesson_id: Option<i64>,       // Link to lesson
    pub course_order: Option<i32>,    // Order in course
    pub file_hash: Option<String>,    // SHA-256 for deduplication
    pub storage_provider: Option<String>, // r2, bunny, s3, local
}

#[derive(Debug, Deserialize)]
pub struct UpdateResourceRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub resource_type: Option<String>,
    pub content_type: Option<String>,
    pub file_url: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub video_platform: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<i64>,
    pub duration: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub trader_id: Option<i64>,
    pub resource_date: Option<String>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_pinned: Option<bool>,
    pub section: Option<String>, // ICT 7: Section for organization
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub difficulty_level: Option<String>,
    // ICT 7 NEW: Access control and linking
    pub access_level: Option<String>,
    pub course_id: Option<i64>,
    pub lesson_id: Option<i64>,
    pub course_order: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ResourceResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub resource_type: String,
    pub content_type: String,
    pub file_url: String,
    pub embed_url: String,
    pub secure_download_url: Option<String>, // ICT 7: Signed URL for secure downloads
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub formatted_size: String,
    pub video_platform: Option<String>,
    pub duration: Option<i32>,
    pub formatted_duration: String,
    pub thumbnail_url: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub trading_room_id: i64,
    pub trader_id: Option<i64>,
    pub resource_date: String,
    pub formatted_date: String,
    pub is_published: bool,
    pub is_featured: bool,
    pub is_pinned: bool,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub difficulty_level: Option<String>,
    pub views_count: i32,
    pub downloads_count: i32,
    pub section: Option<String>, // ICT 7: Section field
    pub created_at: String,
    // ICT 7 NEW: Access control
    pub access_level: String,
    pub requires_premium: bool,
    // ICT 7 NEW: Versioning
    pub version: i32,
    pub has_previous_version: bool,
    pub is_latest_version: bool,
    // ICT 7 NEW: Course/Lesson linking
    pub course_id: Option<i64>,
    pub lesson_id: Option<i64>,
    pub course_order: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub total: i64,
    pub last_page: i64,
}

// ═══════════════════════════════════════════════════════════════════════════
// STOCK LIST TYPES (used by both public and admin sub-modules)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StockList {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub list_type: String, // etf, stock, watchlist, sector
    pub trading_room_id: i64,
    pub symbols: serde_json::Value, // JSON array of {symbol, name, sector, notes}
    pub is_active: bool,
    pub is_featured: bool,
    pub sort_order: i32,
    pub week_of: Option<chrono::NaiveDate>,
    pub created_by: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateStockListRequest {
    pub name: String,
    pub description: Option<String>,
    pub list_type: String,
    pub trading_room_id: i64,
    pub symbols: Vec<StockSymbol>,
    pub is_active: Option<bool>,
    pub is_featured: Option<bool>,
    pub week_of: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockSymbol {
    pub symbol: String,
    pub name: Option<String>,
    pub sector: Option<String>,
    pub notes: Option<String>,
    pub price_target: Option<f64>,
    pub entry_price: Option<f64>,
    pub stop_loss: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct StockListQuery {
    pub room_id: Option<i64>,
    pub list_type: Option<String>,
    pub is_active: Option<bool>,
    pub week_of: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ═══════════════════════════════════════════════════════════════════════════
// MASTER ROUTERS (preserves the pre-split public API exactly)
// ═══════════════════════════════════════════════════════════════════════════

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(public::list_resources))
        .route("/{id_or_slug}", get(public::get_resource))
        .route("/{id}/download", get(download::download_resource))
        .route("/{id}/download", post(public::track_download))
        .route(
            "/{id}/secure-download",
            post(download::generate_secure_download),
        )
        .route("/{id}/versions", get(versioning::get_version_history))
        .route("/by-course/{course_id}", get(public::get_course_resources))
        .route("/by-lesson/{lesson_id}", get(public::get_lesson_resources))
        // ICT 7 NEW: User engagement
        .route("/{id}/track-access", post(public::track_resource_access))
        .route("/recently-accessed", get(public::get_recently_accessed))
        .route("/{id}/favorite", get(public::check_resource_favorite))
        .route("/{id}/favorite", post(public::add_resource_favorite))
        .route("/{id}/favorite", delete(public::remove_resource_favorite))
        .route("/favorites", get(public::get_favorite_resources))
        // ICT 7 NEW: Stock/ETF lists
        .route("/stock-lists", get(public::list_stock_lists))
        .route("/stock-lists/{id}", get(public::get_stock_list))
        .route(
            "/stock-lists/latest/{room_id}",
            get(public::get_latest_watchlist),
        )
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_crud::admin_list_resources))
        .route("/", post(admin_crud::create_resource))
        .route("/{id}", put(admin_crud::update_resource))
        .route("/{id}", delete(admin_crud::delete_resource))
        .route("/{id}/new-version", post(versioning::create_new_version))
        .route("/bulk-create", post(bulk::bulk_create_resources))
        .route("/bulk-update", put(bulk::bulk_update_resources))
        .route("/bulk-delete", delete(bulk::bulk_delete_resources))
        .route("/upload-limits", get(admin_crud::get_upload_limits))
        .route("/validate-upload", post(admin_crud::validate_upload))
        // ICT 7 NEW: Analytics
        .route("/analytics", get(admin_crud::get_resource_analytics))
        .route("/download-logs", get(admin_crud::get_download_logs))
        // ICT 7 NEW: Stock/ETF lists admin
        .route("/stock-lists", post(admin_crud::create_stock_list))
        .route("/stock-lists/{id}", put(admin_crud::update_stock_list))
        .route("/stock-lists/{id}", delete(admin_crud::delete_stock_list))
}
