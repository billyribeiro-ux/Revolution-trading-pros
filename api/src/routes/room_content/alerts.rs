//! Alert handlers (list / create / update / delete / mark_read)
//!
//! Split from `room_content.rs` (R10-B maintainability pass, 2026-05-20).
//! Behavior unchanged: routes are still wired through `mod.rs`'s
//! `public_router()` / `admin_router()` exactly as before.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error, info};

use super::{
    ensure_room_access, CreateAlertRequest, ListQuery, PaginationMeta, RoomAlert,
    UpdateAlertRequest,
};
use crate::cache::{cache_keys, cache_ttl};
use crate::{middleware::admin::AdminUser, models::User, AppState};

/// Cached response for alerts list
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AlertsResponse {
    data: Vec<RoomAlert>,
    meta: PaginationMeta,
}

/// List alerts for a room
/// ICT 7+ Phase 2: Redis cached with 60 second TTL for time-sensitive alerts
pub(super) async fn list_alerts(
    State(state): State<AppState>,
    user: User,
    Path(room_slug): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    ensure_room_access(&state, &user).await?;
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Generate cache key
    let cache_key = cache_keys::alerts(&room_slug, page, per_page);

    // Try to get from cache
    let response = state
        .services
        .cache
        .get_or_fetch(&cache_key, cache_ttl::ALERTS, || async {
            let alerts: Vec<RoomAlert> = sqlx::query_as(
                r"SELECT * FROM room_alerts
                   WHERE room_slug = $1 AND deleted_at IS NULL AND is_published = true
                   ORDER BY is_pinned DESC, published_at DESC
                   LIMIT $2 OFFSET $3",
            )
            .bind(&room_slug)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await?;

            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM room_alerts WHERE room_slug = $1 AND deleted_at IS NULL",
            )
            .bind(&room_slug)
            .fetch_one(&state.db.pool)
            .await?;

            Ok(AlertsResponse {
                data: alerts,
                meta: PaginationMeta {
                    current_page: page,
                    per_page,
                    total: total.0,
                    total_pages: (total.0 as f64 / per_page as f64).ceil() as i64,
                },
            })
        })
        .await
        .map_err(|e| {
            error!("Failed to fetch alerts: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    debug!(
        target: "cache",
        room_slug = %room_slug,
        cache_key = %cache_key,
        "Alerts response served"
    );

    Ok(Json(json!({
        "data": response.data,
        "meta": response.meta
    })))
}

/// Create a new alert with full TOS support
/// ICT 7+ Phase 2: Invalidates alerts cache after creation
pub(super) async fn create_alert(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateAlertRequest>,
) -> Result<Json<RoomAlert>, (StatusCode, Json<serde_json::Value>)> {
    // Parse expiration date if provided
    let expiration = input
        .expiration
        .as_ref()
        .and_then(|e| NaiveDate::parse_from_str(e, "%Y-%m-%d").ok());

    let alert: RoomAlert = sqlx::query_as(
        r"INSERT INTO room_alerts
           (room_id, room_slug, alert_type, ticker, title, message, notes,
            trade_type, action, quantity, option_type, strike, expiration, contract_type,
            order_type, limit_price, fill_price, tos_string, entry_alert_id, trade_plan_id,
            is_new, is_published, is_pinned)
           VALUES (
               (SELECT id FROM trading_rooms WHERE slug = $1 LIMIT 1),
               $1, $2, $3, $4, $5, $6,
               $7, $8, $9, $10, $11, $12, $13,
               $14, $15, $16, $17, $18, $19,
               $20, $21, $22
           )
           RETURNING *",
    )
    .bind(&input.room_slug)
    .bind(input.alert_type.to_uppercase())
    .bind(input.ticker.to_uppercase())
    .bind(&input.title)
    .bind(&input.message)
    .bind(&input.notes)
    .bind(&input.trade_type)
    .bind(input.action.as_ref().map(|a| a.to_uppercase()))
    .bind(input.quantity)
    .bind(input.option_type.as_ref().map(|o| o.to_uppercase()))
    .bind(input.strike)
    .bind(expiration)
    .bind(&input.contract_type)
    .bind(input.order_type.as_ref().map(|o| o.to_uppercase()))
    .bind(input.limit_price)
    .bind(input.fill_price)
    .bind(&input.tos_string)
    .bind(input.entry_alert_id)
    .bind(input.trade_plan_id)
    .bind(input.is_new.unwrap_or(true))
    .bind(input.is_published.unwrap_or(true))
    .bind(input.is_pinned.unwrap_or(false))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create alert: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Invalidate alerts cache
    if let Err(e) = state
        .services
        .cache_invalidator
        .on_alert_change(&input.room_slug, Some(alert.id))
        .await
    {
        error!("Failed to invalidate alerts cache: {}", e);
    }

    // ICT 7+ Phase 3: Broadcast WebSocket event for real-time updates
    state
        .ws_manager
        .broadcast_alert_created(crate::routes::websocket::AlertPayload {
            id: alert.id,
            room_slug: alert.room_slug.clone(),
            alert_type: alert.alert_type.clone(),
            ticker: alert.ticker.clone(),
            title: alert.title.clone(),
            message: alert.message.clone(),
            notes: alert.notes.clone(),
            tos_string: alert.tos_string.clone(),
            is_new: alert.is_new,
            is_pinned: alert.is_pinned,
            published_at: alert.published_at,
            created_at: alert.created_at,
        })
        .await;

    info!(
        "Created alert: {} {} for {} (TOS: {:?})",
        alert.alert_type, alert.ticker, input.room_slug, alert.tos_string
    );
    Ok(Json(alert))
}

/// Update an alert with full TOS support
/// ICT 7+ Phase 2: Invalidates alerts cache after update
pub(super) async fn update_alert(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateAlertRequest>,
) -> Result<Json<RoomAlert>, (StatusCode, Json<serde_json::Value>)> {
    // Parse expiration date if provided
    let expiration = input
        .expiration
        .as_ref()
        .and_then(|e| NaiveDate::parse_from_str(e, "%Y-%m-%d").ok());

    let alert: RoomAlert = sqlx::query_as(
        r"UPDATE room_alerts SET
           alert_type = COALESCE($2, alert_type),
           ticker = COALESCE($3, ticker),
           title = COALESCE($4, title),
           message = COALESCE($5, message),
           notes = COALESCE($6, notes),
           trade_type = COALESCE($7, trade_type),
           action = COALESCE($8, action),
           quantity = COALESCE($9, quantity),
           option_type = COALESCE($10, option_type),
           strike = COALESCE($11, strike),
           expiration = COALESCE($12, expiration),
           contract_type = COALESCE($13, contract_type),
           order_type = COALESCE($14, order_type),
           limit_price = COALESCE($15, limit_price),
           fill_price = COALESCE($16, fill_price),
           tos_string = COALESCE($17, tos_string),
           entry_alert_id = COALESCE($18, entry_alert_id),
           trade_plan_id = COALESCE($19, trade_plan_id),
           is_new = COALESCE($20, is_new),
           is_published = COALESCE($21, is_published),
           is_pinned = COALESCE($22, is_pinned),
           updated_at = NOW()
           WHERE id = $1 AND deleted_at IS NULL
           RETURNING *",
    )
    .bind(id)
    .bind(input.alert_type.map(|t| t.to_uppercase()))
    .bind(input.ticker.map(|t| t.to_uppercase()))
    .bind(&input.title)
    .bind(&input.message)
    .bind(&input.notes)
    .bind(&input.trade_type)
    .bind(input.action.as_ref().map(|a| a.to_uppercase()))
    .bind(input.quantity)
    .bind(input.option_type.as_ref().map(|o| o.to_uppercase()))
    .bind(input.strike)
    .bind(expiration)
    .bind(&input.contract_type)
    .bind(input.order_type.as_ref().map(|o| o.to_uppercase()))
    .bind(input.limit_price)
    .bind(input.fill_price)
    .bind(&input.tos_string)
    .bind(input.entry_alert_id)
    .bind(input.trade_plan_id)
    .bind(input.is_new)
    .bind(input.is_published)
    .bind(input.is_pinned)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to update alert: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Invalidate alerts cache
    if let Err(e) = state
        .services
        .cache_invalidator
        .on_alert_change(&alert.room_slug, Some(id))
        .await
    {
        error!("Failed to invalidate alerts cache: {}", e);
    }

    // ICT 7+ Phase 3: Broadcast WebSocket event for real-time updates
    state
        .ws_manager
        .broadcast_alert_updated(crate::routes::websocket::AlertPayload {
            id: alert.id,
            room_slug: alert.room_slug.clone(),
            alert_type: alert.alert_type.clone(),
            ticker: alert.ticker.clone(),
            title: alert.title.clone(),
            message: alert.message.clone(),
            notes: alert.notes.clone(),
            tos_string: alert.tos_string.clone(),
            is_new: alert.is_new,
            is_pinned: alert.is_pinned,
            published_at: alert.published_at,
            created_at: alert.created_at,
        })
        .await;

    Ok(Json(alert))
}

/// Delete an alert (soft delete)
/// ICT 7+ Phase 2: Invalidates alerts cache after deletion
pub(super) async fn delete_alert(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get room_slug before deletion for cache invalidation
    let entry: Option<(String,)> =
        sqlx::query_as("SELECT room_slug FROM room_alerts WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    sqlx::query("UPDATE room_alerts SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Invalidate alerts cache and broadcast WebSocket event
    if let Some((room_slug,)) = entry {
        if let Err(e) = state
            .services
            .cache_invalidator
            .on_alert_change(&room_slug, Some(id))
            .await
        {
            error!("Failed to invalidate alerts cache: {}", e);
        }

        // ICT 7+ Phase 3: Broadcast WebSocket event for real-time updates
        state
            .ws_manager
            .broadcast_alert_deleted(&room_slug, id)
            .await;
    }

    Ok(Json(json!({"success": true, "message": "Alert deleted"})))
}

/// Mark alert as read (not new)
pub(super) async fn mark_alert_read(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    ensure_room_access(&state, &user).await?;
    sqlx::query("UPDATE room_alerts SET is_new = false WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"success": true})))
}
