//! Admin trading-room handlers — rooms, sections, traders.
//!
//! R27-B5 split: moved verbatim from the pre-split `trading_rooms.rs`.
//! ICT 7 SECURITY: every handler here requires `AdminUser` extraction.
//! `tracing::info!(target: "security_audit", …)` calls and the
//! ICT 7 AUDIT message text are preserved exactly so the audit log
//! query patterns the SecOps team relies on keep matching.

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde_json::json;

use crate::AppState;
use crate::middleware::admin::AdminUser;

use super::fallback::{get_fallback_rooms, get_fallback_sections};
use super::types::{RoomSection, TradersQuery, TradingRoom, TradingRoomsQuery};

/// List all trading rooms (admin) - ICT 7: Requires admin auth
pub(super) async fn admin_list_trading_rooms(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<TradingRoomsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_list_trading_rooms",
        "ICT 7 AUDIT: Admin listing trading rooms"
    );

    let rooms = sqlx::query_as::<_, TradingRoom>(
        r#"
        SELECT id, name, slug, COALESCE(room_type, 'trading') AS "type",
               description, short_description,
               icon, color, thumbnail_url AS logo_url, banner_url AS image_url,
               COALESCE(sort_order, 0) AS sort_order,
               COALESCE(is_active, true) AS is_active,
               COALESCE(is_featured, false) AS is_featured,
               COALESCE(is_public, true) AS is_public,
               available_sections, features, settings AS metadata,
               (created_at AT TIME ZONE 'UTC') AS created_at,
               (updated_at AT TIME ZONE 'UTC') AS updated_at
        FROM trading_rooms
        WHERE ($1::bool IS NULL OR is_active = $1)
        ORDER BY sort_order ASC
        "#,
    )
    .bind(query.active_only)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if !rooms.is_empty() {
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
    } else {
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

/// Get single trading room by slug (admin) - ICT 7: Requires admin auth
pub(super) async fn admin_get_trading_room(
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

    let room_opt = sqlx::query_as::<_, TradingRoom>(
        r#"
        SELECT id, name, slug, COALESCE(room_type, 'trading') AS "type",
               description, short_description,
               icon, color, thumbnail_url AS logo_url, banner_url AS image_url,
               COALESCE(sort_order, 0) AS sort_order,
               COALESCE(is_active, true) AS is_active,
               COALESCE(is_featured, false) AS is_featured,
               COALESCE(is_public, true) AS is_public,
               available_sections, features, settings AS metadata,
               (created_at AT TIME ZONE 'UTC') AS created_at,
               (updated_at AT TIME ZONE 'UTC') AS updated_at
        FROM trading_rooms
        WHERE slug = $1
        "#,
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if let Some(r) = room_opt {
        Ok(Json(json!({
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
        })))
    } else {
        let fallback = get_fallback_rooms();
        if let Some(rooms) = fallback.as_array()
            && let Some(room) = rooms.iter().find(|r| r["slug"] == slug)
        {
            return Ok(Json(json!({
                "success": true,
                "data": room
            })));
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

/// List all sections (admin) - ICT 7: Requires admin auth
pub(super) async fn admin_list_sections(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_list_sections",
        "ICT 7 AUDIT: Admin listing sections"
    );

    let sections = sqlx::query_as::<_, RoomSection>(
        r"
        SELECT id,
               LOWER(REPLACE(title, ' ', '_')) AS section_key,
               title AS name,
               description,
               NULL::text AS icon,
               COALESCE(sort_order, 0) AS sort_order,
               COALESCE(is_active, true) AS is_active,
               NULL::jsonb AS allowed_resource_types,
               NULL::int AS max_items,
               jsonb_build_array(room_slug) AS restricted_to_rooms
        FROM room_sections
        ORDER BY sort_order ASC
        ",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if !sections.is_empty() {
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
    } else {
        Ok(Json(json!({
            "success": true,
            "data": get_fallback_sections()
        })))
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
pub(super) async fn admin_list_traders(
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
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = page.saturating_sub(1).saturating_mul(per_page);

    type TraderRow = (
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
    );

    let rows: Vec<TraderRow> = sqlx::query_as(
        r"
        SELECT t.id, t.user_id, t.name, t.slug, t.title, t.bio,
               t.avatar_url, t.trading_style, t.years_experience,
               COALESCE(t.is_active, true) AS is_active,
               COALESCE(t.is_featured, false) AS is_featured,
               t.created_at
        FROM room_traders t
        WHERE ($1::boolean IS NULL OR t.is_active = $1)
        ORDER BY t.is_featured DESC, t.name ASC
        LIMIT $2 OFFSET $3
        ",
    )
    .bind(query.active_only)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let traders: Vec<serde_json::Value> = rows
        .into_iter()
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
        .collect();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM room_traders WHERE ($1::boolean IS NULL OR is_active = $1)",
    )
    .bind(query.active_only)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

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
