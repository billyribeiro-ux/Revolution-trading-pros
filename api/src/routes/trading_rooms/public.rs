//! Public (unauthenticated) trading-room handlers.
//!
//! R27-B5 split: moved verbatim from the pre-split `trading_rooms.rs`.
//! SQL, fallback behavior, and response shape are preserved exactly so
//! the public storefront / room landing pages keep rendering the same
//! payload.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::AppState;

use super::fallback::{get_fallback_rooms, get_fallback_sections};
use super::types::{RoomSection, TradersQuery, TradingRoom, TradingRoomsQuery};

/// List all trading rooms in correct order
pub(super) async fn list_trading_rooms(
    State(state): State<AppState>,
    Query(query): Query<TradingRoomsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Try to fetch from database first
    let rooms_result = sqlx::query_as::<_, TradingRoom>(
        r"
        SELECT id, name, slug, type, description, short_description,
               icon, color, logo_url, image_url, sort_order,
               is_active, is_featured, is_public,
               available_sections, features, metadata,
               created_at, updated_at
        FROM trading_rooms
        WHERE ($1::bool IS NULL OR is_active = $1)
        ORDER BY sort_order ASC
        ",
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
pub(super) async fn get_trading_room(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let room_result = sqlx::query_as::<_, TradingRoom>(
        r"
        SELECT id, name, slug, type, description, short_description,
               icon, color, logo_url, image_url, sort_order,
               is_active, is_featured, is_public,
               available_sections, features, metadata,
               created_at, updated_at
        FROM trading_rooms
        WHERE slug = $1 AND is_active = true
        ",
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
pub(super) async fn list_sections(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let sections_result = sqlx::query_as::<_, RoomSection>(
        r"
        SELECT id, section_key, name, description, icon, sort_order,
               is_active, allowed_resource_types, max_items, restricted_to_rooms
        FROM room_sections
        WHERE is_active = true
        ORDER BY sort_order ASC
        ",
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
pub(super) async fn list_traders(
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
