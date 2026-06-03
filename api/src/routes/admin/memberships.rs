//! Admin membership-plan and user-membership management — extracted
//! from the original `routes/admin.rs` as part of the R6-B split
//! (2026-05-20).
//!
//! Handler bodies are byte-identical to the pre-split source.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// MEMBERSHIP MANAGEMENT (Admin)
// ═══════════════════════════════════════════════════════════════════════════

/// Membership plan row (reuse from subscriptions). Money is integer cents.
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct MembershipPlanRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub billing_cycle: String,
    pub is_active: bool,
    pub metadata: Option<serde_json::Value>,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// User membership row with plan info
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct AdminUserMembershipRow {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: i64,
    pub plan_name: Option<String>,
    pub plan_slug: Option<String>,
    pub starts_at: chrono::NaiveDateTime,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub cancelled_at: Option<chrono::NaiveDateTime>,
    pub status: String,
    pub payment_provider: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub current_period_start: Option<chrono::NaiveDateTime>,
    pub current_period_end: Option<chrono::NaiveDateTime>,
    pub cancel_at_period_end: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UserMembershipListQuery {
    pub user_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct GrantMembershipRequest {
    pub user_id: i64,
    pub plan_id: i64,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMembershipRequest {
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub status: Option<String>,
    pub cancel_at_period_end: Option<bool>,
}

/// List all membership plans (admin - includes inactive)
/// ICT 11+ FIX: Returns { plans: [...] } format expected by frontend
pub(super) async fn list_all_plans(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let plans: Vec<MembershipPlanRow> = sqlx::query_as(
        r"SELECT id, name, slug, description,
           (price * 100)::BIGINT AS price_cents,
           billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans ORDER BY price ASC",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", "list_all_plans error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    tracing::info!(target: "admin", "list_all_plans returned {} plans", plans.len());

    // ICT 11+ FIX: Wrap in { plans: [...] } format for frontend compatibility
    Ok(Json(json!({ "plans": plans })))
}

/// List user memberships (admin)
pub(super) async fn list_user_memberships(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(query): Query<UserMembershipListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    let memberships: Vec<AdminUserMembershipRow> = sqlx::query_as(
        r"
        SELECT
            um.id, um.user_id, um.plan_id,
            mp.name as plan_name, mp.slug as plan_slug,
            um.starts_at, um.expires_at, um.cancelled_at, um.status,
            um.payment_provider, um.stripe_subscription_id,
            um.current_period_start, um.current_period_end,
            um.cancel_at_period_end, um.created_at, um.updated_at
        FROM user_memberships um
        LEFT JOIN membership_plans mp ON mp.id = um.plan_id
        WHERE ($1::bigint IS NULL OR um.user_id = $1)
          AND ($2::bigint IS NULL OR um.plan_id = $2)
          AND ($3::text IS NULL OR um.status = $3)
        ORDER BY um.created_at DESC
        LIMIT $4 OFFSET $5
        ",
    )
    .bind(query.user_id)
    .bind(query.plan_id)
    .bind(&query.status)
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

    let total: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*) FROM user_memberships um
        WHERE ($1::bigint IS NULL OR um.user_id = $1)
          AND ($2::bigint IS NULL OR um.plan_id = $2)
          AND ($3::text IS NULL OR um.status = $3)
        ",
    )
    .bind(query.user_id)
    .bind(query.plan_id)
    .bind(&query.status)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "data": memberships,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get single user membership (admin)
pub(super) async fn get_user_membership(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<AdminUserMembershipRow>, (StatusCode, Json<serde_json::Value>)> {
    let membership: AdminUserMembershipRow = sqlx::query_as(
        r"
        SELECT
            um.id, um.user_id, um.plan_id,
            mp.name as plan_name, mp.slug as plan_slug,
            um.starts_at, um.expires_at, um.cancelled_at, um.status,
            um.payment_provider, um.stripe_subscription_id,
            um.current_period_start, um.current_period_end,
            um.cancel_at_period_end, um.created_at, um.updated_at
        FROM user_memberships um
        LEFT JOIN membership_plans mp ON mp.id = um.plan_id
        WHERE um.id = $1
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
            Json(json!({"error": "Membership not found"})),
        )
    })?;

    Ok(Json(membership))
}

/// Grant membership to user (admin)
pub(super) async fn grant_membership(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<GrantMembershipRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify user exists
    let user_exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE id = $1")
        .bind(input.user_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if user_exists.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "User not found"})),
        ));
    }

    // Verify plan exists
    let plan: Option<MembershipPlanRow> = sqlx::query_as(
        r"SELECT id, name, slug, description,
           (price * 100)::BIGINT AS price_cents,
           billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1",
    )
    .bind(input.plan_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if plan.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Plan not found"})),
        ));
    }

    let status = input.status.unwrap_or_else(|| "active".to_string());
    let starts_at = input
        .starts_at
        .unwrap_or_else(|| chrono::Utc::now().naive_utc());

    let membership: AdminUserMembershipRow = sqlx::query_as(
        r"
        INSERT INTO user_memberships (user_id, plan_id, starts_at, expires_at, status, payment_provider, cancel_at_period_end, current_period_start, current_period_end, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 'admin', false, $3, $4, NOW(), NOW())
        RETURNING id, user_id, plan_id,
            (SELECT name FROM membership_plans WHERE id = $2) as plan_name,
            (SELECT slug FROM membership_plans WHERE id = $2) as plan_slug,
            starts_at, expires_at, cancelled_at, status, payment_provider, stripe_subscription_id,
            current_period_start, current_period_end, cancel_at_period_end, created_at, updated_at
        "
    )
    .bind(input.user_id)
    .bind(input.plan_id)
    .bind(starts_at)
    .bind(input.expires_at)
    .bind(&status)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "membership": membership,
        "message": "Membership granted successfully"
    })))
}

/// Update user membership (admin) - extend expiration, change status
pub(super) async fn update_user_membership(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateMembershipRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Build UPDATE dynamically
    let mut set_clauses = Vec::new();
    let mut param_count = 1;

    if input.expires_at.is_some() {
        param_count += 1;
        set_clauses.push(format!("expires_at = ${param_count}"));
        set_clauses.push(format!("current_period_end = ${param_count}"));
    }
    if input.status.is_some() {
        param_count += 1;
        set_clauses.push(format!("status = ${param_count}"));
    }
    if input.cancel_at_period_end.is_some() {
        param_count += 1;
        set_clauses.push(format!("cancel_at_period_end = ${param_count}"));
    }

    set_clauses.push("updated_at = NOW()".to_string());

    if set_clauses.len() == 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let sql = format!(
        r"
        UPDATE user_memberships SET {} WHERE id = $1
        RETURNING id, user_id, plan_id,
            (SELECT name FROM membership_plans WHERE id = plan_id) as plan_name,
            (SELECT slug FROM membership_plans WHERE id = plan_id) as plan_slug,
            starts_at, expires_at, cancelled_at, status, payment_provider, stripe_subscription_id,
            current_period_start, current_period_end, cancel_at_period_end, created_at, updated_at
        ",
        set_clauses.join(", ")
    );

    let mut query_builder =
        sqlx::query_as::<_, AdminUserMembershipRow>(sqlx::AssertSqlSafe(sql.as_str())).bind(id);

    if let Some(expires_at) = input.expires_at {
        query_builder = query_builder.bind(expires_at);
    }
    if let Some(ref status) = input.status {
        query_builder = query_builder.bind(status);
    }
    if let Some(cancel_at_period_end) = input.cancel_at_period_end {
        query_builder = query_builder.bind(cancel_at_period_end);
    }

    let membership = query_builder.fetch_one(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "membership": membership,
        "message": "Membership updated successfully"
    })))
}

/// Delete/revoke user membership (admin)
pub(super) async fn revoke_membership(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
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
            Json(json!({"error": "Membership not found"})),
        ));
    }

    Ok(Json(json!({"message": "Membership revoked successfully"})))
}

/// Get memberships for specific user (admin)
pub(super) async fn get_user_memberships_by_user(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<AdminUserMembershipRow>>, (StatusCode, Json<serde_json::Value>)> {
    let memberships: Vec<AdminUserMembershipRow> = sqlx::query_as(
        r"
        SELECT
            um.id, um.user_id, um.plan_id,
            mp.name as plan_name, mp.slug as plan_slug,
            um.starts_at, um.expires_at, um.cancelled_at, um.status,
            um.payment_provider, um.stripe_subscription_id,
            um.current_period_start, um.current_period_end,
            um.cancel_at_period_end, um.created_at, um.updated_at
        FROM user_memberships um
        LEFT JOIN membership_plans mp ON mp.id = um.plan_id
        WHERE um.user_id = $1
        ORDER BY um.created_at DESC
        ",
    )
    .bind(user_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(memberships))
}
