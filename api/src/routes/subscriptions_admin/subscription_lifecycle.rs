//! Subscriptions Admin — Lifecycle transitions
//!
//! `cancel_subscription`, `pause_subscription`, `resume_subscription`,
//! `renew_subscription`. These four share the same shape: status mutation
//! on `user_memberships` + audit log + return updated row.
//!
//! Byte-for-byte structural extract from the pre-split
//! `routes/subscriptions_admin.rs`. SQL strings, audit/tracing events,
//! and AdminUser gating are identical to the source.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::dto::SubscriptionRow;
use crate::{middleware::admin::AdminUser, AppState};

/// Cancel subscription (admin)
pub(super) async fn cancel_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "subscription_cancel", admin_id = %user.id, subscription_id = %id, "Cancelling subscription");

    let subscription: SubscriptionRow = sqlx::query_as(
        r"
        UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW(), updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Subscription not found"}))))?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription cancelled"}),
    ))
}

/// Pause subscription (admin)
pub(super) async fn pause_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "subscription_pause", admin_id = %user.id, subscription_id = %id, "Pausing subscription");

    let subscription: SubscriptionRow = sqlx::query_as(
        r"
        UPDATE user_memberships SET status = 'paused', updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Subscription not found"}))))?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription paused"}),
    ))
}

/// Resume subscription (admin)
pub(super) async fn resume_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "subscription_resume", admin_id = %user.id, subscription_id = %id, "Resuming subscription");

    let subscription: SubscriptionRow = sqlx::query_as(
        r"
        UPDATE user_memberships SET status = 'active', updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Subscription not found"}))))?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription resumed"}),
    ))
}

/// Renew subscription (admin)
pub(super) async fn renew_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "subscription_renew", admin_id = %user.id, subscription_id = %id, "Renewing subscription");

    let subscription: SubscriptionRow = sqlx::query_as(
        r"
        UPDATE user_memberships SET
            status = 'active',
            starts_at = NOW(),
            expires_at = NOW() + INTERVAL '1 month',
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Subscription not found"}))))?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription renewed"}),
    ))
}
