//! Trading Rooms routes - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Provides API endpoints for trading rooms and sections management.
//! ICT 7 SECURITY: Admin endpoints require AdminUser authentication
//! All 6 rooms in correct order:
//! 1. Day Trading Room
//! 2. Swing Trading Room
//! 3. Small Account Mentorship
//! 4. Explosive Swings
//! 5. SPX Profit Pulse
//! 6. High Octane Scanner

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::middleware::admin::AdminUser;
use crate::models::User;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════
// FALLBACK DATA - Used when database is unavailable
// ═══════════════════════════════════════════════════════════════════════════

fn get_fallback_rooms() -> serde_json::Value {
    json!([
        {
            "id": 1,
            "name": "Day Trading Room",
            "slug": "day-trading-room",
            "type": "trading_room",
            "short_description": "Live day trading sessions with real-time trade alerts and daily market analysis.",
            "icon": "chart-line",
            "color": "#3b82f6",
            "sort_order": 1,
            "is_active": true,
            "is_featured": true,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "watchlist", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        },
        {
            "id": 2,
            "name": "Swing Trading Room",
            "slug": "swing-trading-room",
            "type": "trading_room",
            "short_description": "Live swing trading sessions with swing trade alerts and weekly watchlist.",
            "icon": "trending-up",
            "color": "#10b981",
            "sort_order": 2,
            "is_active": true,
            "is_featured": false,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "watchlist", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        },
        {
            "id": 3,
            "name": "Small Account Mentorship",
            "slug": "small-account-mentorship",
            "type": "mentorship",
            "short_description": "Personalized mentorship for growing small trading accounts with proper risk management.",
            "icon": "wallet",
            "color": "#f59e0b",
            "sort_order": 3,
            "is_active": true,
            "is_featured": false,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        },
        {
            "id": 4,
            "name": "Explosive Swings",
            "slug": "explosive-swings",
            "type": "alert_service",
            "short_description": "High momentum swing trade alerts with detailed analysis and risk/reward ratios.",
            "icon": "rocket",
            "color": "#ef4444",
            "sort_order": 4,
            "is_active": true,
            "is_featured": false,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "watchlist", "weekly_alerts", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        },
        {
            "id": 5,
            "name": "SPX Profit Pulse",
            "slug": "spx-profit-pulse",
            "type": "alert_service",
            "short_description": "Premium SPX options alerts for intraday opportunities with high win rate setups.",
            "icon": "activity",
            "color": "#8b5cf6",
            "sort_order": 5,
            "is_active": true,
            "is_featured": false,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        },
        {
            "id": 6,
            "name": "High Octane Scanner",
            "slug": "high-octane-scanner",
            "type": "alert_service",
            "short_description": "Real-time stock scanner with momentum alerts and breakout detection.",
            "icon": "radar",
            "color": "#06b6d4",
            "sort_order": 6,
            "is_active": true,
            "is_featured": false,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        }
    ])
}

fn get_fallback_sections() -> serde_json::Value {
    json!([
        {
            "id": 1,
            "section_key": "introduction",
            "name": "Introduction",
            "description": "Main welcome and overview videos for the room",
            "icon": "video",
            "sort_order": 1,
            "is_active": true,
            "allowed_resource_types": ["video"],
            "max_items": 3,
            "restricted_to_rooms": null
        },
        {
            "id": 2,
            "section_key": "latest_updates",
            "name": "Latest Updates",
            "description": "Recent announcements, news, and updates",
            "icon": "bell",
            "sort_order": 2,
            "is_active": true,
            "allowed_resource_types": ["video", "pdf", "document"],
            "max_items": null,
            "restricted_to_rooms": null
        },
        {
            "id": 3,
            "section_key": "premium_daily_videos",
            "name": "Premium Daily Videos",
            "description": "Daily trading analysis and market commentary",
            "icon": "calendar",
            "sort_order": 3,
            "is_active": true,
            "allowed_resource_types": ["video"],
            "max_items": null,
            "restricted_to_rooms": null
        },
        {
            "id": 4,
            "section_key": "watchlist",
            "name": "Watchlist",
            "description": "Weekly stock watchlist videos and documents",
            "icon": "list-check",
            "sort_order": 4,
            "is_active": true,
            "allowed_resource_types": ["video", "pdf", "spreadsheet"],
            "max_items": null,
            "restricted_to_rooms": null
        },
        {
            "id": 5,
            "section_key": "weekly_alerts",
            "name": "Weekly Alerts",
            "description": "Weekly alert summaries and trade recaps",
            "icon": "alert-triangle",
            "sort_order": 5,
            "is_active": true,
            "allowed_resource_types": ["video"],
            "max_items": null,
            "restricted_to_rooms": ["explosive-swings"]
        },
        {
            "id": 6,
            "section_key": "learning_center",
            "name": "Learning Center",
            "description": "Educational tutorials, guides, and courses",
            "icon": "school",
            "sort_order": 6,
            "is_active": true,
            "allowed_resource_types": ["video", "pdf", "document"],
            "max_items": null,
            "restricted_to_rooms": null
        }
    ])
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// List all trading rooms in correct order
async fn list_trading_rooms(
    State(state): State<AppState>,
    Query(query): Query<TradingRoomsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Try to fetch from database first
    let rooms_result = sqlx::query_as::<_, TradingRoom>(
        r#"
        SELECT id, name, slug, type, description, short_description,
               icon, color, logo_url, image_url, sort_order,
               is_active, is_featured, is_public,
               available_sections, features, metadata,
               created_at, updated_at
        FROM trading_rooms
        WHERE ($1::bool IS NULL OR is_active = $1)
        ORDER BY sort_order ASC
        "#,
    )
    .bind(query.active_only)
    .fetch_all(&state.db.pool)
    .await;

    match rooms_result {
        Ok(rooms) if !rooms.is_empty() => {
            let data: Vec<serde_json::Value> = rooms
                .iter()
                .map(|r| {
                    json!({
                        "id": r.id,
                        "name": r.name,
                        "slug": r.slug,
                        "type": r.room_type,
                        "description": r.description,
                        "short_description": r.short_description,
                        "icon": r.icon,
                        "color": r.color,
                        "logo_url": r.logo_url,
                        "image_url": r.image_url,
                        "sort_order": r.sort_order,
                        "is_active": r.is_active,
                        "is_featured": r.is_featured,
                        "is_public": r.is_public,
                        "available_sections": r.available_sections,
                        "features": r.features,
                        "created_at": r.created_at.to_string(),
                        "updated_at": r.updated_at.to_string()
                    })
                })
                .collect();

            Ok(Json(json!({
                "success": true,
                "data": data,
                "meta": {
                    "current_page": 1,
                    "per_page": 20,
                    "total": data.len(),
                    "total_pages": 1
                }
            })))
        }
        _ => {
            // Fallback to hardcoded data
            let data = get_fallback_rooms();
            let len = data.as_array().map(|a| a.len()).unwrap_or(0);

            Ok(Json(json!({
                "success": true,
                "data": data,
                "meta": {
                    "current_page": 1,
                    "per_page": 20,
                    "total": len,
                    "total_pages": 1
                }
            })))
        }
    }
}

/// Get single trading room by slug
async fn get_trading_room(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let room_result = sqlx::query_as::<_, TradingRoom>(
        r#"
        SELECT id, name, slug, type, description, short_description,
               icon, color, logo_url, image_url, sort_order,
               is_active, is_featured, is_public,
               available_sections, features, metadata,
               created_at, updated_at
        FROM trading_rooms
        WHERE slug = $1 AND is_active = true
        "#,
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await;

    match room_result {
        Ok(Some(r)) => Ok(Json(json!({
            "success": true,
            "data": {
                "id": r.id,
                "name": r.name,
                "slug": r.slug,
                "type": r.room_type,
                "description": r.description,
                "short_description": r.short_description,
                "icon": r.icon,
                "color": r.color,
                "logo_url": r.logo_url,
                "image_url": r.image_url,
                "sort_order": r.sort_order,
                "is_active": r.is_active,
                "is_featured": r.is_featured,
                "is_public": r.is_public,
                "available_sections": r.available_sections,
                "features": r.features,
                "created_at": r.created_at.to_string(),
                "updated_at": r.updated_at.to_string()
            }
        }))),
        Ok(None) => {
            // Try fallback
            let fallback = get_fallback_rooms();
            if let Some(rooms) = fallback.as_array() {
                if let Some(room) = rooms.iter().find(|r| r["slug"] == slug) {
                    return Ok(Json(json!({
                        "success": true,
                        "data": room
                    })));
                }
            }

            Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "Trading room not found"
                })),
            ))
        }
        Err(_) => {
            // Try fallback on error
            let fallback = get_fallback_rooms();
            if let Some(rooms) = fallback.as_array() {
                if let Some(room) = rooms.iter().find(|r| r["slug"] == slug) {
                    return Ok(Json(json!({
                        "success": true,
                        "data": room
                    })));
                }
            }

            Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "Trading room not found"
                })),
            ))
        }
    }
}

/// List all available sections
async fn list_sections(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let sections_result = sqlx::query_as::<_, RoomSection>(
        r#"
        SELECT id, section_key, name, description, icon, sort_order,
               is_active, allowed_resource_types, max_items, restricted_to_rooms
        FROM room_sections
        WHERE is_active = true
        ORDER BY sort_order ASC
        "#,
    )
    .fetch_all(&state.db.pool)
    .await;

    match sections_result {
        Ok(sections) if !sections.is_empty() => {
            let data: Vec<serde_json::Value> = sections
                .iter()
                .map(|s| {
                    json!({
                        "id": s.id,
                        "section_key": s.section_key,
                        "name": s.name,
                        "description": s.description,
                        "icon": s.icon,
                        "sort_order": s.sort_order,
                        "is_active": s.is_active,
                        "allowed_resource_types": s.allowed_resource_types,
                        "max_items": s.max_items,
                        "restricted_to_rooms": s.restricted_to_rooms
                    })
                })
                .collect();

            Ok(Json(json!({
                "success": true,
                "data": data
            })))
        }
        _ => Ok(Json(json!({
            "success": true,
            "data": get_fallback_sections()
        }))),
    }
}

/// List traders (public - may be empty)
async fn list_traders(
    State(_state): State<AppState>,
    Query(_query): Query<TradersQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "success": true,
        "data": [],
        "meta": {
            "current_page": 1,
            "per_page": 20,
            "total": 0,
            "total_pages": 0
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ADMIN HANDLERS - ICT 7: Require AdminUser authentication
// ═══════════════════════════════════════════════════════════════════════════

/// List all trading rooms (admin) - ICT 7: Requires admin auth
async fn admin_list_trading_rooms(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<TradingRoomsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_list_trading_rooms",
        "ICT 7 AUDIT: Admin listing trading rooms"
    );

    let rooms_result = sqlx::query_as::<_, TradingRoom>(
        r#"
        SELECT id, name, slug, type, description, short_description,
               icon, color, logo_url, image_url, sort_order,
               is_active, is_featured, is_public,
               available_sections, features, metadata,
               created_at, updated_at
        FROM trading_rooms
        WHERE ($1::bool IS NULL OR is_active = $1)
        ORDER BY sort_order ASC
        "#,
    )
    .bind(query.active_only)
    .fetch_all(&state.db.pool)
    .await;

    match rooms_result {
        Ok(rooms) if !rooms.is_empty() => {
            let data: Vec<serde_json::Value> = rooms
                .iter()
                .map(|r| {
                    json!({
                        "id": r.id,
                        "name": r.name,
                        "slug": r.slug,
                        "type": r.room_type,
                        "description": r.description,
                        "short_description": r.short_description,
                        "icon": r.icon,
                        "color": r.color,
                        "logo_url": r.logo_url,
                        "image_url": r.image_url,
                        "sort_order": r.sort_order,
                        "is_active": r.is_active,
                        "is_featured": r.is_featured,
                        "is_public": r.is_public,
                        "available_sections": r.available_sections,
                        "features": r.features,
                        "metadata": r.metadata,
                        "created_at": r.created_at.to_string(),
                        "updated_at": r.updated_at.to_string()
                    })
                })
                .collect();

            Ok(Json(json!({
                "success": true,
                "data": data,
                "meta": {
                    "current_page": 1,
                    "per_page": 20,
                    "total": data.len(),
                    "total_pages": 1
                }
            })))
        }
        _ => {
            let data = get_fallback_rooms();
            let len = data.as_array().map(|a| a.len()).unwrap_or(0);

            Ok(Json(json!({
                "success": true,
                "data": data,
                "meta": {
                    "current_page": 1,
                    "per_page": 20,
                    "total": len,
                    "total_pages": 1
                }
            })))
        }
    }
}

/// Get single trading room by slug (admin) - ICT 7: Requires admin auth
async fn admin_get_trading_room(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_get_trading_room",
        slug = %slug,
        "ICT 7 AUDIT: Admin fetching trading room"
    );

    let room_result = sqlx::query_as::<_, TradingRoom>(
        r#"
        SELECT id, name, slug, type, description, short_description,
               icon, color, logo_url, image_url, sort_order,
               is_active, is_featured, is_public,
               available_sections, features, metadata,
               created_at, updated_at
        FROM trading_rooms
        WHERE slug = $1
        "#,
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await;

    match room_result {
        Ok(Some(r)) => Ok(Json(json!({
            "success": true,
            "data": {
                "id": r.id,
                "name": r.name,
                "slug": r.slug,
                "type": r.room_type,
                "description": r.description,
                "short_description": r.short_description,
                "icon": r.icon,
                "color": r.color,
                "logo_url": r.logo_url,
                "image_url": r.image_url,
                "sort_order": r.sort_order,
                "is_active": r.is_active,
                "is_featured": r.is_featured,
                "is_public": r.is_public,
                "available_sections": r.available_sections,
                "features": r.features,
                "metadata": r.metadata,
                "created_at": r.created_at.to_string(),
                "updated_at": r.updated_at.to_string()
            }
        }))),
        Ok(None) => {
            let fallback = get_fallback_rooms();
            if let Some(rooms) = fallback.as_array() {
                if let Some(room) = rooms.iter().find(|r| r["slug"] == slug) {
                    return Ok(Json(json!({
                        "success": true,
                        "data": room
                    })));
                }
            }
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "Trading room not found"
                })),
            ))
        }
        Err(_) => {
            let fallback = get_fallback_rooms();
            if let Some(rooms) = fallback.as_array() {
                if let Some(room) = rooms.iter().find(|r| r["slug"] == slug) {
                    return Ok(Json(json!({
                        "success": true,
                        "data": room
                    })));
                }
            }
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "Trading room not found"
                })),
            ))
        }
    }
}

/// List all sections (admin) - ICT 7: Requires admin auth
async fn admin_list_sections(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_list_sections",
        "ICT 7 AUDIT: Admin listing sections"
    );

    let sections_result = sqlx::query_as::<_, RoomSection>(
        r#"
        SELECT id, section_key, name, description, icon, sort_order,
               is_active, allowed_resource_types, max_items, restricted_to_rooms
        FROM room_sections
        ORDER BY sort_order ASC
        "#,
    )
    .fetch_all(&state.db.pool)
    .await;

    match sections_result {
        Ok(sections) if !sections.is_empty() => {
            let data: Vec<serde_json::Value> = sections
                .iter()
                .map(|s| {
                    json!({
                        "id": s.id,
                        "section_key": s.section_key,
                        "name": s.name,
                        "description": s.description,
                        "icon": s.icon,
                        "sort_order": s.sort_order,
                        "is_active": s.is_active,
                        "allowed_resource_types": s.allowed_resource_types,
                        "max_items": s.max_items,
                        "restricted_to_rooms": s.restricted_to_rooms
                    })
                })
                .collect();

            Ok(Json(json!({
                "success": true,
                "data": data
            })))
        }
        _ => Ok(Json(json!({
            "success": true,
            "data": get_fallback_sections()
        }))),
    }
}

/// List traders (admin) - ICT 7: Requires admin auth
///
/// FIX (audit 2026-05-16, TRADING_ROOMS_BACKEND_GAPS Gap 1): was a stub
/// returning hardcoded `data: []`, so the admin Traders list rendered
/// empty once the frontend proxy's `mockTraders` fallback was removed by
/// P1-11. Now queries `room_traders` (schema: migrations/015 §2),
/// mirroring the proven `admin_list_videos` pattern: tuple `query_as`,
/// `.map().unwrap_or_default()`, separate COUNT, identical `meta` shape.
///
/// `room_traders` has no room foreign key (rooms reference traders via
/// `instructor_id`/`creator_id`, not the reverse), so `room_slug` has no
/// column to filter on and is intentionally not applied here — only the
/// filters that map to real columns (`active_only`) plus pagination, the
/// same "honor what the schema supports" approach as `admin_list_videos`.
async fn admin_list_traders(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<TradersQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_list_traders",
        "ICT 7 AUDIT: Admin listing traders"
    );
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let traders: Vec<serde_json::Value> = sqlx::query_as::<
        _,
        (
            i64,
            Option<i64>,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<i32>,
            bool,
            bool,
            chrono::DateTime<chrono::Utc>,
        ),
    >(
        r#"
        SELECT t.id, t.user_id, t.name, t.slug, t.title, t.bio,
               t.avatar_url, t.trading_style, t.years_experience,
               t.is_active, t.is_featured, t.created_at
        FROM room_traders t
        WHERE ($1::boolean IS NULL OR t.is_active = $1)
        ORDER BY t.is_featured DESC, t.name ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(query.active_only)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|r| {
                json!({
                    "id": r.0,
                    "user_id": r.1,
                    "name": r.2,
                    "slug": r.3,
                    "title": r.4,
                    "bio": r.5,
                    "avatar_url": r.6,
                    "trading_style": r.7,
                    "years_experience": r.8,
                    "is_active": r.9,
                    "is_featured": r.10,
                    "created_at": r.11.to_string()
                })
            })
            .collect()
    })
    .unwrap_or_default();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM room_traders WHERE ($1::boolean IS NULL OR is_active = $1)",
    )
    .bind(query.active_only)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);

    Ok(Json(json!({
        "success": true,
        "data": traders,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total,
            "total_pages": (total as f64 / per_page as f64).ceil() as i64
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// VIDEO HANDLERS (Admin) - ICT 7 FIX: Add /admin/trading-rooms/videos routes
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct VideosQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub room_slug: Option<String>,
    pub content_type: Option<String>,
    pub is_published: Option<bool>,
}

/// List videos for trading rooms (admin) - ICT 7: Requires admin auth
async fn admin_list_videos(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<VideosQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_list_videos",
        "ICT 7 AUDIT: Admin listing videos"
    );
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Query videos with optional room filter
    let videos: Vec<serde_json::Value> = sqlx::query_as::<
        _,
        (
            i64,
            String,
            String,
            Option<String>,
            String,
            Option<String>,
            Option<i32>,
            String,
            bool,
            chrono::NaiveDateTime,
        ),
    >(
        r#"
        SELECT v.id, v.title, v.slug, v.description, v.video_url, v.thumbnail_url, 
               v.duration, v.content_type, v.is_published, v.created_at
        FROM unified_videos v
        WHERE v.deleted_at IS NULL
          AND ($1::text IS NULL OR v.content_type = $1)
          AND ($2::boolean IS NULL OR v.is_published = $2)
        ORDER BY v.created_at DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(&query.content_type)
    .bind(query.is_published)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|r| {
                json!({
                    "id": r.0,
                    "title": r.1,
                    "slug": r.2,
                    "description": r.3,
                    "video_url": r.4,
                    "thumbnail_url": r.5,
                    "duration": r.6,
                    "content_type": r.7,
                    "is_published": r.8,
                    "created_at": r.9.to_string()
                })
            })
            .collect()
    })
    .unwrap_or_default();

    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM unified_videos WHERE deleted_at IS NULL")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or(0);

    Ok(Json(json!({
        "success": true,
        "data": videos,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total,
            "total_pages": (total as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// List videos for a specific trading room by slug (admin) - ICT 7: Requires admin auth
async fn admin_list_videos_by_room(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(slug): Path<String>,
    Query(query): Query<VideosQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_list_videos_by_room",
        room_slug = %slug,
        "ICT 7 AUDIT: Admin listing videos by room"
    );
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Get room ID from slug
    let room_id: Option<i64> = sqlx::query_scalar("SELECT id FROM trading_rooms WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();

    let videos: Vec<serde_json::Value> = if let Some(rid) = room_id {
        sqlx::query_as::<
            _,
            (
                i64,
                String,
                String,
                Option<String>,
                String,
                Option<String>,
                Option<i32>,
                String,
                bool,
                chrono::NaiveDateTime,
            ),
        >(
            r#"
            SELECT v.id, v.title, v.slug, v.description, v.video_url, v.thumbnail_url, 
                   v.duration, v.content_type, v.is_published, v.created_at
            FROM unified_videos v
            JOIN video_room_assignments vra ON v.id = vra.video_id
            WHERE vra.trading_room_id = $1
              AND v.deleted_at IS NULL
              AND ($2::text IS NULL OR v.content_type = $2)
              AND ($3::boolean IS NULL OR v.is_published = $3)
            ORDER BY v.created_at DESC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(rid)
        .bind(&query.content_type)
        .bind(query.is_published)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|r| {
                    json!({
                        "id": r.0,
                        "title": r.1,
                        "slug": r.2,
                        "description": r.3,
                        "video_url": r.4,
                        "thumbnail_url": r.5,
                        "duration": r.6,
                        "content_type": r.7,
                        "is_published": r.8,
                        "created_at": r.9.to_string()
                    })
                })
                .collect()
        })
        .unwrap_or_default()
    } else {
        vec![]
    };

    let total = videos.len() as i64;

    Ok(Json(json!({
        "success": true,
        "data": videos,
        "room_slug": slug,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total,
            "total_pages": (total as f64 / per_page as f64).ceil().max(1.0) as i64
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROOMS STATS (Admin Dashboard)
// ═══════════════════════════════════════════════════════════════════════════

/// Per-room content counts returned to /admin/dashboard.
///
/// FIX-2026-04-26: shape matches `RoomStats` interface in
/// `routes/admin/dashboard/+page.svelte` exactly. The page expects an
/// array (one entry per room) and matches by `room_id` (which it
/// populates from `ROOMS[*].id`, i.e. the room slug). Counts that
/// can't be sourced for a given room default to zero.
#[derive(Debug, Serialize)]
struct RoomStatsRow {
    room_id: String,
    watchlist_count: i64,
    modules_count: i64,
    articles_count: i64,
}

/// Get per-room content stats for the admin dashboard.
/// GET /api/admin/rooms/stats
///
/// Sources each count via dedicated `COALESCE(SUM/COUNT, 0)` queries on
/// `watchlist_entries`, `unified_videos`, and `room_resources` joined by
/// `room_slug`. Each room enumerated from `trading_rooms` (with the same
/// hardcoded fallback list as `list_trading_rooms`) gets a row, with zeros
/// for any subquery that fails — but unlike `unwrap_or_default()`, errors
/// are propagated via `?` so the endpoint surfaces real DB failures
/// (FIX-2026-04-26 error-propagation pattern).
async fn admin_get_rooms_stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Vec<RoomStatsRow>>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_get_rooms_stats",
        "ICT 7 AUDIT: Admin fetching rooms stats"
    );

    // Collect the canonical room-slug list from `trading_rooms` if available,
    // otherwise fall back to the hardcoded room slugs used elsewhere.
    let db_slugs: Vec<String> = sqlx::query_scalar::<_, String>(
        r#"SELECT slug FROM trading_rooms WHERE is_active = true ORDER BY sort_order ASC"#,
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let room_slugs: Vec<String> = if db_slugs.is_empty() {
        get_fallback_rooms()
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|r| r.get("slug").and_then(|s| s.as_str()).map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    } else {
        db_slugs
    };

    let mut stats: Vec<RoomStatsRow> = Vec::with_capacity(room_slugs.len());

    for slug in room_slugs {
        let watchlist_count: i64 = sqlx::query_scalar(
            r#"SELECT COALESCE(COUNT(*), 0)::BIGINT
               FROM watchlist_entries
               WHERE room_slug = $1 AND is_active = true"#,
        )
        .bind(&slug)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "trading_rooms", room_slug = %slug, "watchlist_count query failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("watchlist_count query failed: {}", e)})),
            )
        })?;

        let modules_count: i64 = sqlx::query_scalar(
            r#"SELECT COALESCE(COUNT(*), 0)::BIGINT
               FROM unified_videos
               WHERE room_slug = $1
                 AND is_published = true
                 AND deleted_at IS NULL"#,
        )
        .bind(&slug)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "trading_rooms", room_slug = %slug, "modules_count query failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("modules_count query failed: {}", e)})),
            )
        })?;

        // "Articles" maps to non-video room resources (PDFs, docs, etc.).
        let articles_count: i64 = sqlx::query_scalar(
            r#"SELECT COALESCE(COUNT(*), 0)::BIGINT
               FROM room_resources
               WHERE room_slug = $1
                 AND is_published = true
                 AND deleted_at IS NULL
                 AND resource_type <> 'video'"#,
        )
        .bind(&slug)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "trading_rooms", room_slug = %slug, "articles_count query failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("articles_count query failed: {}", e)})),
            )
        })?;

        stats.push(RoomStatsRow {
            room_id: slug,
            watchlist_count,
            modules_count,
            articles_count,
        });
    }

    Ok(Json(stats))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTERS
// ═══════════════════════════════════════════════════════════════════════════

/// Public router for trading rooms
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_trading_rooms))
        .route("/sections", get(list_sections))
        .route("/:slug", get(get_trading_room))
        .route("/traders", get(list_traders))
}

/// Admin router for trading rooms - ICT 7: All endpoints require AdminUser authentication
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_trading_rooms))
        .route("/sections", get(admin_list_sections))
        .route("/:slug", get(admin_get_trading_room))
        .route("/traders", get(admin_list_traders))
        // ICT 7 FIX: Add videos routes for frontend compatibility
        .route("/videos", get(admin_list_videos))
        .route("/videos/:slug", get(admin_list_videos_by_room))
}

/// Admin "rooms" router — short-prefix companion to `admin_router()` so
/// `/api/admin/rooms/stats` resolves without colliding with
/// `/api/admin/trading-rooms/...`.
///
/// FIX-2026-04-26: added to back the dashboard's
/// `fetch('/api/admin/rooms/stats')` call.
pub fn admin_rooms_router() -> Router<AppState> {
    Router::new().route("/stats", get(admin_get_rooms_stats))
}
