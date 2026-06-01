//! Public plan-listing endpoints (no auth required).
//!
//! R15-B (2026-05-20): extracted from the original 1,690-LOC
//! `subscriptions.rs` as a pure structural move. All SQL, error
//! mapping, and money handling preserved byte-for-byte.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::dtos::{MembershipPlanExtended, MembershipPlanRow};
use crate::AppState;

/// List all membership plans (public)
pub(super) async fn list_plans(
    State(state): State<AppState>,
) -> Result<Json<Vec<MembershipPlanRow>>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 11+ Fix: Cast DECIMAL price to FLOAT8 for SQLx f64 compatibility
    let plans: Vec<MembershipPlanRow> = sqlx::query_as(
        r"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE is_active = true ORDER BY price ASC",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(plans))
}

/// Get plan by slug (public)
pub(super) async fn get_plan(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<MembershipPlanRow>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 11+ Fix: Cast DECIMAL price to FLOAT8 for SQLx f64 compatibility
    let plan: MembershipPlanRow = sqlx::query_as(
        r"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE slug = $1",
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Plan not found"})),
        )
    })?;

    Ok(Json(plan))
}

/// Get all subscription plan variants for a specific trading room
/// GET /subscriptions/room/:room_slug/plans
pub(super) async fn get_room_plans(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let plans: Vec<MembershipPlanExtended> = sqlx::query_as(
        r"SELECT
            mp.id, mp.name, mp.slug, mp.display_name, mp.description,
            (mp.price * 100)::BIGINT AS price_cents, mp.billing_cycle,
            mp.interval_count, mp.savings_percent, mp.is_popular,
            mp.is_active, mp.metadata, mp.stripe_price_id, mp.stripe_product_id,
            mp.features, mp.trial_days, mp.sort_order, mp.room_id,
            tr.name as room_name, tr.slug as room_slug
        FROM membership_plans mp
        JOIN trading_rooms tr ON tr.id = mp.room_id
        WHERE tr.slug = $1 AND mp.is_active = true
        ORDER BY mp.sort_order ASC",
    )
    .bind(&room_slug)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if plans.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "No plans found for this room"})),
        ));
    }

    // Get room info from first plan
    let room_name = plans
        .first()
        .and_then(|p| p.room_name.clone())
        .unwrap_or_default();

    Ok(Json(json!({
        "room_slug": room_slug,
        "room_name": room_name,
        "plans": plans,
        "total": plans.len()
    })))
}
