//! Subscriptions Admin — Membership plan CRUD + stats
//!
//! `list_plans`, `get_plan`, `create_plan`, `update_plan`, `delete_plan`,
//! `plan_stats`. Money: `price_cents` is `i64` end-to-end; SQL converts
//! between `BIGINT` cents at the API boundary and `NUMERIC` `price`
//! column via `$N::BIGINT / 100.0` for writes and `(price * 100)::BIGINT
//! AS price_cents` for reads — preserved byte-for-byte from the
//! pre-split file.
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

use super::dto::{CreatePlanRequest, PlanListQuery, SubscriptionPlanRow, UpdatePlanRequest};
use crate::{middleware::admin::AdminUser, AppState};

/// List all subscription plans (admin)
pub(super) async fn list_plans(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Query(query): Query<PlanListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // ICT 11+ Fix: Cast DECIMAL price to FLOAT8 for SQLx f64 compatibility
    let plans: Vec<SubscriptionPlanRow> = sqlx::query_as(
        r"
        SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle, is_active,
               stripe_price_id, features, trial_days, trial_period_days,
               trial_requires_payment_method, created_at, updated_at
        FROM membership_plans
        WHERE ($1::boolean IS NULL OR is_active = $1)
        ORDER BY price ASC
        LIMIT $2 OFFSET $3
        ",
    )
    .bind(query.is_active)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in list_plans: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // FIX-2026-04-26 (audit 02 §P1-11): propagate errors instead of `0`.
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM membership_plans WHERE ($1::boolean IS NULL OR is_active = $1)",
    )
    .bind(query.is_active)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "list_plans", error = %e, "total count query failed");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    Ok(Json(json!({
        "data": plans,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get single plan (admin)
pub(super) async fn get_plan(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 11+ Fix: Cast DECIMAL price to FLOAT8 for SQLx f64 compatibility
    let plan: SubscriptionPlanRow = sqlx::query_as(
        r"
        SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle, is_active,
               stripe_price_id, features, trial_days, trial_period_days,
               trial_requires_payment_method, created_at, updated_at
        FROM membership_plans WHERE id = $1
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
            Json(json!({"error": "Plan not found"})),
        )
    })?;

    Ok(Json(json!({"data": plan})))
}

/// Create subscription plan (admin)
pub(super) async fn create_plan(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreatePlanRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "plan_create",
        admin_id = %user.id,
        plan_name = %input.name,
        "Admin creating subscription plan"
    );

    let slug = slug::slugify(&input.name);
    let features = input.features.unwrap_or(json!([]));

    // Cents at the API surface; convert to NUMERIC at SQL boundary
    let plan: SubscriptionPlanRow = sqlx::query_as(
        r"
        INSERT INTO membership_plans (name, slug, description, price, billing_cycle, is_active, stripe_price_id, features, trial_days, trial_period_days, trial_requires_payment_method, created_at, updated_at)
        VALUES ($1, $2, $3, $4::BIGINT / 100.0, $5, $6, $7, $8, $9, $10, $11, NOW(), NOW())
        RETURNING id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle, is_active, stripe_price_id, features, trial_days, trial_period_days, trial_requires_payment_method, created_at, updated_at
        "
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(input.price_cents)
    .bind(&input.billing_cycle)
    .bind(input.is_active.unwrap_or(true))
    .bind(&input.stripe_price_id)
    .bind(&features)
    .bind(input.trial_days)
    .bind(input.trial_period_days)
    .bind(input.trial_requires_payment_method.unwrap_or(true))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create plan: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    Ok(Json(
        json!({"data": plan, "message": "Plan created successfully"}),
    ))
}

/// Update subscription plan (admin)
pub(super) async fn update_plan(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdatePlanRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "plan_update",
        admin_id = %user.id,
        plan_id = %id,
        "Admin updating subscription plan"
    );

    let plan: SubscriptionPlanRow = sqlx::query_as(
        r"
        UPDATE membership_plans SET
            name = COALESCE($2, name),
            slug = CASE WHEN $2 IS NOT NULL THEN LOWER(REPLACE($2, ' ', '-')) ELSE slug END,
            description = COALESCE($3, description),
            price = COALESCE($4::BIGINT / 100.0, price),
            billing_cycle = COALESCE($5, billing_cycle),
            is_active = COALESCE($6, is_active),
            stripe_price_id = COALESCE($7, stripe_price_id),
            features = COALESCE($8, features),
            trial_days = COALESCE($9, trial_days),
            trial_period_days = COALESCE($10, trial_period_days),
            trial_requires_payment_method = COALESCE($11, trial_requires_payment_method),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle, is_active, stripe_price_id, features, trial_days, trial_period_days, trial_requires_payment_method, created_at, updated_at
        "
    )
    .bind(id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(input.price_cents)
    .bind(&input.billing_cycle)
    .bind(input.is_active)
    .bind(&input.stripe_price_id)
    .bind(&input.features)
    .bind(input.trial_days)
    .bind(input.trial_period_days)
    .bind(input.trial_requires_payment_method)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Plan not found"}))))?;

    Ok(Json(
        json!({"data": plan, "message": "Plan updated successfully"}),
    ))
}

/// Delete subscription plan (admin)
pub(super) async fn delete_plan(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "plan_delete",
        admin_id = %user.id,
        plan_id = %id,
        "Admin deleting subscription plan"
    );

    let result = sqlx::query("DELETE FROM membership_plans WHERE id = $1")
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
            Json(json!({"error": "Plan not found"})),
        ));
    }

    Ok(Json(json!({"message": "Plan deleted successfully"})))
}

/// Get plan stats (admin)
///
/// FIX-2026-04-26 (audit 02 §P1-11): the previous body used
/// `.unwrap_or((0,))` on three separate queries. A real DB outage or schema
/// drift would silently render `0/0/0` in the dashboard — masked as
/// "membership plans operating normally". Per CLAUDE.md ("Don't swallow
/// errors with `unwrap_or_default()` on `Result<T, E>` — propagate via `?`")
/// we now propagate via `?` and log on failure so the frontend gets a 500.
pub(super) async fn plan_stats(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total_plans: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM membership_plans")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "plan_stats", error = %e, "total_plans query failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to load plan stats"})),
            )
        })?;

    let active_plans: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM membership_plans WHERE is_active = true")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!(target: "plan_stats", error = %e, "active_plans query failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to load plan stats"})),
                )
            })?;

    let total_subscriptions: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'active'")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!(target: "plan_stats", error = %e, "active subscriptions count query failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to load plan stats"})),
                )
            })?;

    Ok(Json(json!({
        "data": {
            "total_plans": total_plans.0,
            "active_plans": active_plans.0,
            "total_active_subscriptions": total_subscriptions.0
        }
    })))
}
