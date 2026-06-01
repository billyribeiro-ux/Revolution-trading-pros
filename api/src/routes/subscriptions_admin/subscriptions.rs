//! Subscriptions Admin — CRUD handlers
//!
//! `list_subscriptions`, `get_subscription`, `create_subscription`,
//! `update_subscription`, `delete_subscription`.
//!
//! Byte-for-byte structural extract from the pre-split
//! `routes/subscriptions_admin.rs`. SQL strings, audit/tracing events,
//! and AdminUser gating are identical to the source.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::dto::{
    CreateSubscriptionRequest, SubscriptionListQuery, SubscriptionRow, UpdateSubscriptionRequest,
};
use crate::{middleware::admin::AdminUser, AppState};

/// List all subscriptions (admin)
pub(super) async fn list_subscriptions(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Query(query): Query<SubscriptionListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let subscriptions: Vec<SubscriptionRow> = sqlx::query_as(
        r"
        SELECT id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
               payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        FROM user_memberships
        WHERE ($1::text IS NULL OR status = $1)
          AND ($2::bigint IS NULL OR user_id = $2)
          AND ($3::bigint IS NULL OR plan_id = $3)
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        ",
    )
    .bind(query.status.as_deref())
    .bind(query.user_id)
    .bind(query.plan_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in list_subscriptions: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // FIX-2026-04-26 (audit 02 §P1-11): propagate DB errors instead of
    // silently rendering `total: 0` on connection drop.
    let total: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*) FROM user_memberships
        WHERE ($1::text IS NULL OR status = $1)
          AND ($2::bigint IS NULL OR user_id = $2)
          AND ($3::bigint IS NULL OR plan_id = $3)
        ",
    )
    .bind(query.status.as_deref())
    .bind(query.user_id)
    .bind(query.plan_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "list_subscriptions", error = %e, "total count query failed");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    Ok(Json(json!({
        "data": subscriptions,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get single subscription (admin)
pub(super) async fn get_subscription(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let subscription: SubscriptionRow = sqlx::query_as(
        r"
        SELECT id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
               payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        FROM user_memberships WHERE id = $1
        ",
    )
    .bind(id)
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
            Json(json!({"error": "Subscription not found"})),
        )
    })?;

    Ok(Json(json!({"data": subscription})))
}

/// Create subscription (admin)
pub(super) async fn create_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateSubscriptionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "subscription_create",
        admin_id = %user.id,
        user_id = %input.user_id,
        plan_id = %input.plan_id,
        "Admin creating subscription"
    );

    let status = input.status.as_deref().unwrap_or("active");

    let subscription: SubscriptionRow = sqlx::query_as(
        r"
        INSERT INTO user_memberships (user_id, plan_id, status, starts_at, expires_at, created_at, updated_at)
        VALUES ($1, $2, $3, COALESCE($4::timestamp, NOW()), $5::timestamp, NOW(), NOW())
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "
    )
    .bind(input.user_id)
    .bind(input.plan_id)
    .bind(status)
    .bind(&input.starts_at)
    .bind(&input.expires_at)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create subscription: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription created successfully"}),
    ))
}

/// Update subscription (admin)
pub(super) async fn update_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateSubscriptionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "subscription_update",
        admin_id = %user.id,
        subscription_id = %id,
        "Admin updating subscription"
    );

    let subscription: SubscriptionRow = sqlx::query_as(
        r"
        UPDATE user_memberships SET
            status = COALESCE($2, status),
            plan_id = COALESCE($3, plan_id),
            starts_at = COALESCE($4::timestamp, starts_at),
            expires_at = COALESCE($5::timestamp, expires_at),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "
    )
    .bind(id)
    .bind(&input.status)
    .bind(input.plan_id)
    .bind(&input.starts_at)
    .bind(&input.expires_at)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Subscription not found"}))))?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription updated successfully"}),
    ))
}

/// Delete subscription (admin)
pub(super) async fn delete_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "subscription_delete",
        admin_id = %user.id,
        subscription_id = %id,
        "Admin deleting subscription"
    );

    let result = sqlx::query("DELETE FROM user_memberships WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Subscription not found"})),
        ));
    }

    Ok(Json(
        json!({"message": "Subscription deleted successfully"}),
    ))
}
