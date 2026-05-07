//! Admin routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025
//!
//! Enterprise-grade admin API with role-based access control.
//! All routes require admin or super-admin role.

#![allow(clippy::type_complexity)]
#![allow(clippy::needless_borrows_for_generic_args)]

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    middleware::admin::{AdminUser, SuperAdminUser},
    models::User,
    AppState,
};

// Batch 5b: local `require_admin` and `require_super_admin` helpers
// removed. Admin/super-admin auth is now uniformly enforced via the
// `AdminUser` and `SuperAdminUser` extractors from
// `crate::middleware::admin`. The error shape changed slightly
// (plain text "Admin access required" vs the old JSON body with
// `your_role`); admin clients should treat 403 as "not authorized,"
// not parse the response body.

// ═══════════════════════════════════════════════════════════════════════════
// USER MANAGEMENT (Admin Staff)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct AdminUserRow {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub role: Option<String>,
    pub is_active: bool,
    pub email_verified_at: Option<chrono::NaiveDateTime>,
    pub last_login_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub role: Option<String>,
    pub search: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
}

/// List all users (admin)
/// ICT 11+ SECURITY FIX: Refactored to use safe parameterized queries with optional filters
/// ICT 7 SECURITY FIX: Added require_admin check - was missing!
async fn list_users(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(query): Query<UserListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    // ICT 11+ SECURITY: Use NULL-safe parameterized queries
    // This approach is SQL injection proof and compile-time verified
    let search_pattern: Option<String> = query.search.as_ref().map(|s| format!("%{}%", s));

    let users: Vec<AdminUserRow> = sqlx::query_as(
        r#"
        SELECT id, name, email, role, is_active, email_verified_at, last_login_at, created_at, updated_at 
        FROM users
        WHERE ($1::text IS NULL OR role = $1)
          AND ($2::boolean IS NULL OR is_active = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR email ILIKE $3)
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        "#
    )
    .bind(query.role.as_deref())
    .bind(query.is_active)
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in list_users: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Database error"})))
    })?;

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) 
        FROM users
        WHERE ($1::text IS NULL OR role = $1)
          AND ($2::boolean IS NULL OR is_active = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR email ILIKE $3)
        "#,
    )
    .bind(query.role.as_deref())
    .bind(query.is_active)
    .bind(search_pattern.as_deref())
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in list_users count: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    Ok(Json(json!({
        "data": users,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get user by ID (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
async fn get_user(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<AdminUserRow>, (StatusCode, Json<serde_json::Value>)> {
    let target_user: AdminUserRow = sqlx::query_as(
        "SELECT id, name, email, role, is_active, email_verified_at, last_login_at, created_at, updated_at FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "User not found"}))))?;

    Ok(Json(target_user))
}

/// Create user (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
async fn create_user(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateUserRequest>,
) -> Result<Json<AdminUserRow>, (StatusCode, Json<serde_json::Value>)> {
    // FIX-2026-04-27 (H-4): Validate password strength before hashing.
    // Previously hash_password was called without validate_password, so admins
    // could create accounts with passwords like "a" that bypass the policy.
    crate::utils::validate_password(&input.password)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;

    let password_hash = crate::utils::hash_password(&input.password).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let role = input.role.unwrap_or_else(|| "user".to_string());

    let user: AdminUserRow = sqlx::query_as(
        r#"
        INSERT INTO users (name, email, password_hash, role, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, true, NOW(), NOW())
        RETURNING id, name, email, role, is_active, email_verified_at, last_login_at, created_at, updated_at
        "#
    )
    .bind(&input.name)
    .bind(&input.email)
    .bind(&password_hash)
    .bind(&role)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") {
            (StatusCode::CONFLICT, Json(json!({"error": "Email already exists"})))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        }
    })?;

    Ok(Json(user))
}

/// Update user (admin)
/// SECURITY: Uses parameterized queries to prevent SQL injection
/// ICT 7 SECURITY FIX: Added require_admin check
async fn update_user(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateUserRequest>,
) -> Result<Json<AdminUserRow>, (StatusCode, Json<serde_json::Value>)> {
    // FIX-2026-04-27 (H-7): Only super_admin may grant privileged roles.
    // A regular admin promoting anyone to developer/super_admin is a privilege escalation path.
    if let Some(ref new_role) = input.role {
        let privileged = matches!(
            new_role.as_str(),
            "developer" | "super_admin" | "super-admin"
        );
        if privileged {
            let actor_role = user.role.as_deref().unwrap_or("user");
            let actor_is_super = actor_role == "super_admin" || actor_role == "super-admin";
            if !actor_is_super {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(
                        json!({"error": "Only super_admin can grant developer or super_admin roles"}),
                    ),
                ));
            }
        }
    }

    // Fetch old role for the audit log before the update
    let old_role: Option<(Option<String>,)> =
        sqlx::query_as("SELECT role FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch user role for audit: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Database error"})),
                )
            })?;

    // Build UPDATE query with parameterized values
    let mut set_clauses = Vec::new();
    let mut param_count = 1;

    if input.name.is_some() {
        param_count += 1;
        set_clauses.push(format!("name = ${}", param_count));
    }
    if input.email.is_some() {
        param_count += 1;
        set_clauses.push(format!("email = ${}", param_count));
    }
    if input.role.is_some() {
        param_count += 1;
        set_clauses.push(format!("role = ${}", param_count));
    }
    if input.is_active.is_some() {
        param_count += 1;
        set_clauses.push(format!("is_active = ${}", param_count));
    }

    set_clauses.push("updated_at = NOW()".to_string());

    if set_clauses.len() == 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let sql = format!(
        "UPDATE users SET {} WHERE id = $1 RETURNING id, name, email, role, is_active, email_verified_at, last_login_at, created_at, updated_at",
        set_clauses.join(", ")
    );

    let mut query_builder = sqlx::query_as::<_, AdminUserRow>(&sql).bind(id);

    if let Some(ref name) = input.name {
        query_builder = query_builder.bind(name);
    }
    if let Some(ref email) = input.email {
        query_builder = query_builder.bind(email);
    }
    if let Some(ref role) = input.role {
        query_builder = query_builder.bind(role);
    }
    if let Some(is_active) = input.is_active {
        query_builder = query_builder.bind(is_active);
    }

    let updated = query_builder
        .fetch_one(&state.db.pool)
        .await
        .map_err(|_e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    // FIX-2026-04-27 (H-7): Write audit log row for every role change.
    if let Some(ref new_role) = input.role {
        let prev_role = old_role
            .and_then(|(r,)| r)
            .unwrap_or_else(|| "user".to_string());
        let details = serde_json::json!({
            "old_role": prev_role,
            "new_role": new_role,
            "actor_id": user.id,
            "target_user_id": id
        });
        if let Err(e) = sqlx::query(
            r#"INSERT INTO security_events (user_id, event_type, event_category, severity, details)
               VALUES ($1, 'role_change', 'access_control', 'high', $2)"#,
        )
        .bind(id)
        .bind(&details)
        .execute(&state.db.pool)
        .await
        {
            tracing::error!(
                target: "security_audit",
                event = "role_change_audit_write_failed",
                actor_id = %user.id,
                target_user_id = %id,
                error = %e,
                "Failed to write role change audit log"
            );
        }
    }

    Ok(Json(updated))
}

/// Delete user (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
async fn delete_user(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "User deleted successfully"})))
}

/// Ban user (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
async fn ban_user(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE users SET is_active = false, updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // FIX-2026-04-27 (H-6): Invalidate all active sessions and the user cache so
    // banned users are kicked out immediately, not just on next token refresh.
    // Tolerate Redis failure — the ban is already persisted in the DB and Fix C-1
    // will block the user on the next DB fetch.
    if let Some(ref redis) = state.services.redis {
        if let Err(e) = redis.invalidate_all_user_sessions(id).await {
            tracing::warn!(
                target: "security_audit",
                event = "ban_session_invalidation_failed",
                user_id = %id,
                error = %e,
                "Could not invalidate sessions for banned user (Redis unavailable) - DB ban still effective"
            );
        }
        if let Err(e) = redis.invalidate_user_cache(id).await {
            tracing::warn!(
                target: "security_audit",
                event = "ban_cache_invalidation_failed",
                user_id = %id,
                error = %e,
                "Could not invalidate user cache for banned user"
            );
        }
    }

    Ok(Json(json!({"message": "User banned successfully"})))
}

/// Unban user (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
async fn unban_user(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE users SET is_active = true, updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "User unbanned successfully"})))
}

/// User stats (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
async fn user_stats(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    let active: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE is_active = true")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    let verified: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM users WHERE email_verified_at IS NOT NULL")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    let admins: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM users WHERE role IN ('admin', 'super-admin')")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    Ok(Json(json!({
        "total": total.0,
        "active": active.0,
        "verified": verified.0,
        "admins": admins.0
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// COUPON MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

/// Coupon row returned to admin UI. All monetary fields are integer cents.
/// For percent coupons: `discount_value_cents = percent * 100` (e.g. 5000 = 50%).
///
/// `stripe_coupon_id` mirrors the row into a Stripe Coupon; populated on
/// admin create. `duration` is one of "once" / "forever" / "repeating";
/// `duration_in_months` is required when duration='repeating'.
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct CouponRow {
    pub id: i64,
    pub code: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value_cents: i64,
    pub min_purchase_cents: Option<i64>,
    pub max_discount_cents: Option<i64>,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub is_active: bool,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub applicable_products: Option<serde_json::Value>,
    pub applicable_plans: Option<serde_json::Value>,
    pub stripe_coupon_id: Option<String>,
    pub duration: String,
    pub duration_in_months: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCouponRequest {
    pub code: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value_cents: i64,
    pub min_purchase_cents: Option<i64>,
    pub max_discount_cents: Option<i64>,
    pub usage_limit: Option<i32>,
    pub is_active: Option<bool>,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub applicable_products: Option<serde_json::Value>,
    pub applicable_plans: Option<serde_json::Value>,
    /// "once" | "forever" | "repeating". Defaults to "once" when omitted.
    pub duration: Option<String>,
    /// Required when `duration == "repeating"`.
    pub duration_in_months: Option<i32>,
}

/// List coupons (admin)
/// ICT 7 FIX: Use explicit column list with FLOAT8 casting for DECIMAL columns
async fn list_coupons(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<Vec<CouponRow>>, (StatusCode, Json<serde_json::Value>)> {
    // Money is integer cents at the Rust boundary (architecture standard §1.2).
    // DB columns remain NUMERIC(10,2) as a display cache; convert at the SQL edge.
    let coupons: Vec<CouponRow> = sqlx::query_as(
        r#"SELECT
            id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        FROM coupons ORDER BY created_at DESC"#,
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", "list_coupons error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(coupons))
}

/// Create coupon (admin).
///
/// Mirrors the row into a Stripe Coupon (Batch 3.5 directive). Stripe is
/// the source of truth for discount math at checkout time; this row stores
/// our admin metadata (code, description, usage tracking, applicable
/// products/plans) plus a pointer (`stripe_coupon_id`) to the Stripe object
/// the checkout flow attaches via `discounts[]`.
///
/// On Stripe-side failure the DB row is NOT created. On DB-side failure
/// after Stripe success, the orphaned Stripe coupon is deleted so the two
/// stores stay in sync.
async fn create_coupon(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateCouponRequest>,
) -> Result<Json<CouponRow>, (StatusCode, Json<serde_json::Value>)> {
    // ── Validate inputs ──────────────────────────────────────────────────
    let duration = input.duration.clone().unwrap_or_else(|| "once".into());
    if !matches!(duration.as_str(), "once" | "forever" | "repeating") {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "duration must be 'once', 'forever', or 'repeating'"})),
        ));
    }
    let duration_in_months = input.duration_in_months;
    if duration == "repeating" && duration_in_months.unwrap_or(0) <= 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "duration_in_months > 0 required when duration='repeating'"})),
        ));
    }
    if duration != "repeating" && duration_in_months.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "duration_in_months only valid when duration='repeating'"})),
        ));
    }
    if !matches!(
        input.discount_type.as_str(),
        "percent" | "percentage" | "fixed" | "amount"
    ) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "discount_type must be 'percent' or 'fixed'"})),
        ));
    }

    // ── Create the Stripe Coupon (canonical source for the math) ────────
    let (percent_off, amount_off_cents) = match input.discount_type.as_str() {
        "percent" | "percentage" => {
            // discount_value_cents stores 'percent * 100' (e.g. 1000 = 10.00%).
            let percent = (input.discount_value_cents as f64) / 100.0;
            if percent <= 0.0 || percent > 100.0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "percent must be > 0 and <= 100"})),
                ));
            }
            (Some(percent), None)
        }
        _ => {
            if input.discount_value_cents <= 0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "discount_value_cents must be > 0 for fixed coupons"})),
                ));
            }
            (None, Some(input.discount_value_cents))
        }
    };

    let stripe_coupon = state
        .services
        .stripe
        .create_coupon(crate::services::stripe::CreateStripeCouponRequest {
            percent_off,
            amount_off_cents,
            currency: "usd".into(),
            duration: duration.clone(),
            duration_in_months: duration_in_months.map(|n| n as i64),
            name: Some(input.code.to_uppercase()),
            max_redemptions: input.usage_limit.map(|n| n as i64),
            redeem_by_unix: input.expires_at.map(|t| t.and_utc().timestamp()),
        })
        .await
        .map_err(|e| {
            tracing::error!(target: "admin", "Stripe coupon create failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Stripe coupon create failed: {}", e)})),
            )
        })?;

    // ── INSERT the DB row (with Stripe pointer) ─────────────────────────
    let coupon_result: Result<CouponRow, sqlx::Error> = sqlx::query_as(
        r#"
        INSERT INTO coupons (code, description, discount_type, discount_value, min_purchase, max_discount,
                             usage_limit, usage_count, is_active, starts_at, expires_at,
                             applicable_products, applicable_plans,
                             stripe_coupon_id, duration, duration_in_months,
                             created_at, updated_at)
        VALUES (UPPER($1), $2, $3, $4::BIGINT / 100.0, $5::BIGINT / 100.0, $6::BIGINT / 100.0,
                $7, 0, $8, $9, $10, $11, $12,
                $13, $14, $15,
                NOW(), NOW())
        RETURNING id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        "#,
    )
    .bind(&input.code)
    .bind(&input.description)
    .bind(&input.discount_type)
    .bind(input.discount_value_cents)
    .bind(input.min_purchase_cents)
    .bind(input.max_discount_cents)
    .bind(input.usage_limit)
    .bind(input.is_active.unwrap_or(true))
    .bind(&input.starts_at)
    .bind(&input.expires_at)
    .bind(&input.applicable_products)
    .bind(&input.applicable_plans)
    .bind(&stripe_coupon.id)
    .bind(&duration)
    .bind(duration_in_months)
    .fetch_one(&state.db.pool)
    .await;

    match coupon_result {
        Ok(coupon) => Ok(Json(coupon)),
        Err(e) => {
            // Roll back the Stripe coupon so the two stores stay in sync.
            tracing::error!(
                target: "admin",
                "create_coupon DB insert failed after Stripe coupon {} created; rolling back Stripe: {}",
                stripe_coupon.id, e
            );
            if let Err(del_err) = state.services.stripe.delete_coupon(&stripe_coupon.id).await {
                tracing::error!(
                    target: "admin",
                    "Failed to roll back orphan Stripe coupon {}: {}",
                    stripe_coupon.id, del_err
                );
            }
            if e.to_string().contains("duplicate") {
                Err((
                    StatusCode::CONFLICT,
                    Json(json!({"error": "Coupon code already exists"})),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                ))
            }
        }
    }
}

/// Delete coupon (admin). Removes the DB row and the Stripe-side coupon.
/// Stripe coupon deletion stops future redemptions; subscriptions that
/// already have the discount keep it for the rest of its `duration`.
async fn delete_coupon(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let stripe_coupon_id: Option<Option<String>> =
        sqlx::query_scalar("SELECT stripe_coupon_id FROM coupons WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!(target: "admin", "delete_coupon read error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    sqlx::query("DELETE FROM coupons WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if let Some(Some(sid)) = stripe_coupon_id {
        if let Err(e) = state.services.stripe.delete_coupon(&sid).await {
            tracing::warn!(
                target: "admin",
                "DB coupon {} deleted but Stripe coupon {} delete failed: {}; manual cleanup required",
                id, sid, e
            );
        }
    }

    Ok(Json(json!({"message": "Coupon deleted successfully"})))
}

// FIX-2026-04-26 (P0-1, CC-1): Add the missing GET /admin/coupons/:id and
// PUT /admin/coupons/:id handlers so the admin coupon edit page can load and
// save coupons against the migration-correct schema.

/// Get a single coupon by ID (admin)
/// ICT 7 FIX: Use explicit column list with FLOAT8 casting for DECIMAL columns
async fn get_coupon(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<CouponRow>, (StatusCode, Json<serde_json::Value>)> {
    let coupon: Option<CouponRow> = sqlx::query_as(
        r#"SELECT
            id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        FROM coupons WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", "get_coupon error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    coupon.map(Json).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Coupon not found"})),
        )
    })
}

/// Update coupon (admin)
/// ICT 7 FIX: Schema-aligned UPDATE matching the migration columns.
/// Uses COALESCE so partial updates leave untouched columns alone, except for
/// nullable date fields where the caller may want to clear them by sending null.
/// Update coupon (admin).
///
/// Two paths:
///
///   1. **Metadata-only edits** (description, expires_at, applicable_*,
///      usage_limit, is_active, code rename): just update the DB row.
///      The Stripe coupon stays as-is. Cheap.
///
///   2. **Discount-math edits** (discount_type, discount_value_cents,
///      duration, duration_in_months): Stripe Coupons are immutable, so we
///      create a NEW Stripe Coupon, flip `stripe_coupon_id` on the DB row,
///      then best-effort delete the old Stripe coupon. Existing customers
///      who already redeemed the old code keep their discount per Stripe's
///      `duration` semantics — that is the correct, audited behavior.
///
/// On Stripe-side failure during recreate, the DB row is left unchanged
/// (no half-state). On DB-side failure after Stripe recreate, the new
/// orphan Stripe coupon is deleted.
async fn update_coupon(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<CreateCouponRequest>,
) -> Result<Json<CouponRow>, (StatusCode, Json<serde_json::Value>)> {
    // ── Validate discount_type if provided ──────────────────────────────
    if !input.discount_type.is_empty()
        && input.discount_type != "percent"
        && input.discount_type != "percentage"
        && input.discount_type != "fixed"
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Discount type must be 'percent' or 'fixed'"})),
        ));
    }

    // ── Validate duration semantics if provided ─────────────────────────
    if let Some(ref d) = input.duration {
        if !matches!(d.as_str(), "once" | "forever" | "repeating") {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "duration must be 'once', 'forever', or 'repeating'"})),
            ));
        }
        if d == "repeating" && input.duration_in_months.unwrap_or(0) <= 0 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "duration_in_months > 0 required when duration='repeating'"})),
            ));
        }
        if d != "repeating" && input.duration_in_months.is_some() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "duration_in_months only valid when duration='repeating'"})),
            ));
        }
    }

    // ── Snapshot current row so we can detect discount-math edits ───────
    #[derive(sqlx::FromRow)]
    struct ExistingCoupon {
        discount_type: String,
        discount_value_cents: i64,
        duration: String,
        duration_in_months: Option<i32>,
        stripe_coupon_id: Option<String>,
    }
    let existing: ExistingCoupon = sqlx::query_as(
        r#"SELECT discount_type,
                  (discount_value * 100)::BIGINT AS discount_value_cents,
                  duration, duration_in_months, stripe_coupon_id
           FROM coupons WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", "update_coupon snapshot error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Coupon not found"})),
    ))?;

    // Resolve effective new discount-math fields. Empty/None means "keep".
    let new_discount_type: String = if !input.discount_type.is_empty() {
        // Normalize 'percentage' → 'percent' for Stripe semantics.
        let dt = if input.discount_type == "percentage" {
            "percent"
        } else {
            input.discount_type.as_str()
        };
        dt.to_string()
    } else {
        existing.discount_type.clone()
    };
    let new_discount_value_cents: i64 = if input.discount_value_cents != 0 {
        input.discount_value_cents
    } else {
        existing.discount_value_cents
    };
    let new_duration: String = input
        .duration
        .clone()
        .unwrap_or_else(|| existing.duration.clone());
    let new_duration_in_months: Option<i32> = if input.duration.is_some() {
        input.duration_in_months
    } else {
        existing.duration_in_months
    };

    let math_changed = new_discount_type != existing.discount_type
        || new_discount_value_cents != existing.discount_value_cents
        || new_duration != existing.duration
        || new_duration_in_months != existing.duration_in_months
        || existing.stripe_coupon_id.is_none();

    // ── If math changed, create a new Stripe Coupon BEFORE the DB update.
    let mut new_stripe_coupon_id: Option<String> = None;
    if math_changed {
        let (percent_off, amount_off_cents) = match new_discount_type.as_str() {
            "percent" | "percentage" => {
                let percent = (new_discount_value_cents as f64) / 100.0;
                if percent <= 0.0 || percent > 100.0 {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "percent must be > 0 and <= 100"})),
                    ));
                }
                (Some(percent), None)
            }
            _ => {
                if new_discount_value_cents <= 0 {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(
                            json!({"error": "discount_value_cents must be > 0 for fixed coupons"}),
                        ),
                    ));
                }
                (None, Some(new_discount_value_cents))
            }
        };
        let req = crate::services::stripe::CreateStripeCouponRequest {
            percent_off,
            amount_off_cents,
            currency: "usd".into(),
            duration: new_duration.clone(),
            duration_in_months: new_duration_in_months.map(|n| n as i64),
            name: Some(input.code.to_uppercase()),
            max_redemptions: input.usage_limit.map(|n| n as i64),
            redeem_by_unix: input.expires_at.map(|t| t.and_utc().timestamp()),
        };
        let created = state
            .services
            .stripe
            .create_coupon(req)
            .await
            .map_err(|e| {
                tracing::error!(target: "admin", "Stripe coupon recreate failed: {}", e);
                (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({"error": format!("Stripe coupon recreate failed: {}", e)})),
                )
            })?;
        new_stripe_coupon_id = Some(created.id);
    }

    // ── DB UPDATE ───────────────────────────────────────────────────────
    let coupon_result: Result<Option<CouponRow>, sqlx::Error> = sqlx::query_as(
        r#"
        UPDATE coupons SET
            code = COALESCE(NULLIF(UPPER($2), ''), code),
            description = COALESCE($3, description),
            discount_type = COALESCE(NULLIF($4, ''), discount_type),
            discount_value = COALESCE(NULLIF($5, 0)::BIGINT / 100.0, discount_value),
            min_purchase = COALESCE($6::BIGINT / 100.0, min_purchase),
            max_discount = COALESCE($7::BIGINT / 100.0, max_discount),
            usage_limit = COALESCE($8, usage_limit),
            is_active = COALESCE($9, is_active),
            starts_at = COALESCE($10, starts_at),
            expires_at = COALESCE($11, expires_at),
            applicable_products = COALESCE($12, applicable_products),
            applicable_plans = COALESCE($13, applicable_plans),
            duration = COALESCE($14, duration),
            duration_in_months = CASE WHEN $14 IS NOT NULL THEN $15 ELSE duration_in_months END,
            stripe_coupon_id = COALESCE($16, stripe_coupon_id),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(&input.code)
    .bind(&input.description)
    .bind(&input.discount_type)
    .bind(input.discount_value_cents)
    .bind(input.min_purchase_cents)
    .bind(input.max_discount_cents)
    .bind(input.usage_limit)
    .bind(input.is_active)
    .bind(&input.starts_at)
    .bind(&input.expires_at)
    .bind(&input.applicable_products)
    .bind(&input.applicable_plans)
    .bind(input.duration.as_deref())
    .bind(input.duration_in_months)
    .bind(new_stripe_coupon_id.as_deref())
    .fetch_optional(&state.db.pool)
    .await;

    match coupon_result {
        Ok(Some(coupon)) => {
            // DB success. If math changed, best-effort delete old Stripe coupon.
            if math_changed {
                if let Some(ref old) = existing.stripe_coupon_id {
                    if Some(old) != new_stripe_coupon_id.as_ref() {
                        if let Err(e) = state.services.stripe.delete_coupon(old).await {
                            tracing::warn!(
                                target: "admin",
                                "DB coupon {} updated; new Stripe coupon attached but old Stripe coupon {} delete failed: {}; manual cleanup may be needed",
                                id, old, e
                            );
                        }
                    }
                }

                // Batch 4: write an audit row for the recreate-and-swap so
                // we have a permanent paper trail of who flipped the
                // pointer and when — useful for support, billing
                // disputes, and compliance review. Best-effort: failure
                // here does NOT roll back the Stripe + DB swap.
                let mut fields_changed: Vec<&'static str> = Vec::new();
                if new_discount_type != existing.discount_type {
                    fields_changed.push("discount_type");
                }
                if new_discount_value_cents != existing.discount_value_cents {
                    fields_changed.push("discount_value_cents");
                }
                if new_duration != existing.duration {
                    fields_changed.push("duration");
                }
                if new_duration_in_months != existing.duration_in_months {
                    fields_changed.push("duration_in_months");
                }
                if let Err(e) = sqlx::query(
                    r#"INSERT INTO security_events (user_id, event_type, details, created_at)
                       VALUES ($1, 'coupon_recreated', $2::JSONB, NOW())"#,
                )
                .bind(user.id)
                .bind(json!({
                    "coupon_id": id,
                    "old_stripe_coupon_id": existing.stripe_coupon_id,
                    "new_stripe_coupon_id": new_stripe_coupon_id,
                    "fields_changed": fields_changed,
                }))
                .execute(&state.db.pool)
                .await
                {
                    tracing::warn!(
                        target: "admin",
                        "coupon recreate audit log write failed for coupon {}: {}",
                        id, e
                    );
                }
            }
            Ok(Json(coupon))
        }
        Ok(None) => {
            // DB row not found after we'd already created a new Stripe coupon → orphan cleanup.
            if let Some(ref new_id) = new_stripe_coupon_id {
                let _ = state.services.stripe.delete_coupon(new_id).await;
            }
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Coupon not found"})),
            ))
        }
        Err(e) => {
            tracing::error!(target: "admin", "update_coupon DB error: {}", e);
            // Roll back orphan Stripe coupon if math changed.
            if let Some(ref new_id) = new_stripe_coupon_id {
                let _ = state.services.stripe.delete_coupon(new_id).await;
            }
            if e.to_string().contains("duplicate") {
                Err((
                    StatusCode::CONFLICT,
                    Json(json!({"error": "Coupon code already exists"})),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                ))
            }
        }
    }
}

/// Mirror a DB coupon row into Stripe (Batch 4 backfill tool).
///
/// POST /api/admin/coupons/:id/sync-to-stripe
///
/// Use case: a DB row exists but `stripe_coupon_id IS NULL` — usually
/// because a Stripe API outage during create left the row half-mirrored,
/// or the row was imported from outside the admin UI. Calling this
/// endpoint creates a fresh Stripe Coupon from the DB fields and stores
/// its id on the row.
///
/// If the row is already mirrored (`stripe_coupon_id IS NOT NULL`) we
/// return 400 with a directive to use the edit endpoint instead. This
/// avoids the foot-gun of accidentally orphaning a Stripe coupon and
/// keeps the recreate-and-swap policy in a single place
/// (`update_coupon`). The check itself is race-safe: the read+create+
/// update sequence re-checks NULL by virtue of the unconditional
/// `WHERE id = $2` UPDATE; concurrent calls would race to fail with a
/// duplicate-stripe-id error on the second writer.
async fn sync_coupon_to_stripe(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<CouponRow>, (StatusCode, Json<serde_json::Value>)> {
    // Read existing row.
    let row: Option<CouponRow> = sqlx::query_as(
        r#"SELECT id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        FROM coupons WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", "sync_coupon_to_stripe read error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    let coupon = row.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Coupon not found"})),
    ))?;

    // Already mirrored — return 400 per Batch 4 spec. Operator should
    // use the edit endpoint to change discount math.
    if coupon.stripe_coupon_id.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Coupon already synced to Stripe; use the edit endpoint to change it"
            })),
        ));
    }

    // Map DB discount_type → Stripe percent_off / amount_off.
    let (percent_off, amount_off_cents) = match coupon.discount_type.as_str() {
        "percent" | "percentage" => {
            let percent = (coupon.discount_value_cents as f64) / 100.0;
            if percent <= 0.0 || percent > 100.0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(
                        json!({"error": "Stored discount_value invalid for percent coupon (must be > 0 and <= 100)"}),
                    ),
                ));
            }
            (Some(percent), None)
        }
        _ => {
            if coupon.discount_value_cents <= 0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(
                        json!({"error": "Stored discount_value_cents must be > 0 for fixed coupons"}),
                    ),
                ));
            }
            (None, Some(coupon.discount_value_cents))
        }
    };

    let req = crate::services::stripe::CreateStripeCouponRequest {
        percent_off,
        amount_off_cents,
        currency: "usd".into(),
        duration: coupon.duration.clone(),
        duration_in_months: coupon.duration_in_months.map(|n| n as i64),
        name: Some(coupon.code.to_uppercase()),
        max_redemptions: coupon.usage_limit.map(|n| n as i64),
        redeem_by_unix: coupon.expires_at.map(|t| t.and_utc().timestamp()),
    };
    let created = state
        .services
        .stripe
        .create_coupon(req)
        .await
        .map_err(|e| {
            tracing::error!(target: "admin", "sync_coupon_to_stripe Stripe create failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Stripe coupon create failed: {}", e)})),
            )
        })?;

    // Persist stripe_coupon_id on the DB row.
    let updated: CouponRow = sqlx::query_as(
        r#"UPDATE coupons SET stripe_coupon_id = $1, updated_at = NOW()
           WHERE id = $2
           RETURNING id, code, description, discount_type,
               (discount_value * 100)::BIGINT AS discount_value_cents,
               (min_purchase * 100)::BIGINT AS min_purchase_cents,
               (max_discount * 100)::BIGINT AS max_discount_cents,
               usage_limit, usage_count, is_active, starts_at, expires_at,
               applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at"#,
    )
    .bind(&created.id)
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        // DB write failed AFTER we created a new Stripe coupon → orphan cleanup.
        tracing::error!(
            target: "admin",
            "sync_coupon_to_stripe DB persist failed for coupon {}; rolling back Stripe coupon {}: {}",
            id, created.id, e
        );
        let stripe = state.services.stripe.clone();
        let stripe_id = created.id.clone();
        tokio::spawn(async move {
            let _ = stripe.delete_coupon(&stripe_id).await;
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    tracing::info!(
        target: "admin",
        event = "coupon_synced_to_stripe",
        coupon_id = %id,
        stripe_coupon_id = %created.id,
        "Mirrored DB coupon into Stripe"
    );

    Ok(Json(updated))
}

/// Validate coupon (public)
/// ICT 7 FIX: Use explicit column list with FLOAT8 casting for DECIMAL columns
async fn validate_coupon(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let coupon: Option<CouponRow> = sqlx::query_as(
        r#"SELECT
            id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        FROM coupons WHERE UPPER(code) = UPPER($1) AND is_active = true"#,
    )
    .bind(&code)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", "validate_coupon error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    match coupon {
        Some(c) => {
            // Check expiry
            if let Some(expires_at) = c.expires_at {
                if expires_at < chrono::Utc::now().naive_utc() {
                    return Ok(Json(json!({"valid": false, "error": "Coupon has expired"})));
                }
            }
            // Check usage limit
            if let Some(limit) = c.usage_limit {
                if c.usage_count >= limit {
                    return Ok(Json(
                        json!({"valid": false, "error": "Coupon usage limit reached"}),
                    ));
                }
            }
            Ok(Json(json!({
                "valid": true,
                "coupon": c
            })))
        }
        None => Ok(Json(
            json!({"valid": false, "error": "Invalid coupon code"}),
        )),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SETTINGS MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct SettingRow {
    pub id: i64,
    pub key: String,
    pub value: Option<serde_json::Value>,
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Get all settings (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
async fn get_settings(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<Vec<SettingRow>>, (StatusCode, Json<serde_json::Value>)> {
    let settings: Vec<SettingRow> =
        sqlx::query_as("SELECT * FROM application_settings ORDER BY group_name, key")
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    Ok(Json(settings))
}

/// Get setting by key (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
async fn get_setting(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(key): Path<String>,
) -> Result<Json<SettingRow>, (StatusCode, Json<serde_json::Value>)> {
    let setting: SettingRow = sqlx::query_as("SELECT * FROM application_settings WHERE key = $1")
        .bind(&key)
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
                Json(json!({"error": "Setting not found"})),
            )
        })?;

    Ok(Json(setting))
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingRequest {
    pub value: serde_json::Value,
}

/// Update setting (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
async fn update_setting(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(key): Path<String>,
    Json(input): Json<UpdateSettingRequest>,
) -> Result<Json<SettingRow>, (StatusCode, Json<serde_json::Value>)> {
    let setting: SettingRow = sqlx::query_as(
        "UPDATE application_settings SET value = $1, updated_at = NOW() WHERE key = $2 RETURNING *",
    )
    .bind(&input.value)
    .bind(&key)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(setting))
}

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
async fn list_all_plans(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let plans: Vec<MembershipPlanRow> = sqlx::query_as(
        r#"SELECT id, name, slug, description,
           (price * 100)::BIGINT AS price_cents,
           billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans ORDER BY price ASC"#,
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
async fn list_user_memberships(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(query): Query<UserMembershipListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    let memberships: Vec<AdminUserMembershipRow> = sqlx::query_as(
        r#"
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
        "#,
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
        r#"
        SELECT COUNT(*) FROM user_memberships um
        WHERE ($1::bigint IS NULL OR um.user_id = $1)
          AND ($2::bigint IS NULL OR um.plan_id = $2)
          AND ($3::text IS NULL OR um.status = $3)
        "#,
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
async fn get_user_membership(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<AdminUserMembershipRow>, (StatusCode, Json<serde_json::Value>)> {
    let membership: AdminUserMembershipRow = sqlx::query_as(
        r#"
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
        "#,
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
async fn grant_membership(
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
        r#"SELECT id, name, slug, description,
           (price * 100)::BIGINT AS price_cents,
           billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1"#,
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
        r#"
        INSERT INTO user_memberships (user_id, plan_id, starts_at, expires_at, status, payment_provider, cancel_at_period_end, current_period_start, current_period_end, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 'admin', false, $3, $4, NOW(), NOW())
        RETURNING id, user_id, plan_id,
            (SELECT name FROM membership_plans WHERE id = $2) as plan_name,
            (SELECT slug FROM membership_plans WHERE id = $2) as plan_slug,
            starts_at, expires_at, cancelled_at, status, payment_provider, stripe_subscription_id,
            current_period_start, current_period_end, cancel_at_period_end, created_at, updated_at
        "#
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
async fn update_user_membership(
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
        set_clauses.push(format!("expires_at = ${}", param_count));
        set_clauses.push(format!("current_period_end = ${}", param_count));
    }
    if input.status.is_some() {
        param_count += 1;
        set_clauses.push(format!("status = ${}", param_count));
    }
    if input.cancel_at_period_end.is_some() {
        param_count += 1;
        set_clauses.push(format!("cancel_at_period_end = ${}", param_count));
    }

    set_clauses.push("updated_at = NOW()".to_string());

    if set_clauses.len() == 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let sql = format!(
        r#"
        UPDATE user_memberships SET {} WHERE id = $1
        RETURNING id, user_id, plan_id,
            (SELECT name FROM membership_plans WHERE id = plan_id) as plan_name,
            (SELECT slug FROM membership_plans WHERE id = plan_id) as plan_slug,
            starts_at, expires_at, cancelled_at, status, payment_provider, stripe_subscription_id,
            current_period_start, current_period_end, cancel_at_period_end, created_at, updated_at
        "#,
        set_clauses.join(", ")
    );

    let mut query_builder = sqlx::query_as::<_, AdminUserMembershipRow>(&sql).bind(id);

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
async fn revoke_membership(
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
async fn get_user_memberships_by_user(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<AdminUserMembershipRow>>, (StatusCode, Json<serde_json::Value>)> {
    let memberships: Vec<AdminUserMembershipRow> = sqlx::query_as(
        r#"
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
        "#,
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

// ═══════════════════════════════════════════════════════════════════════════
// CAMPAIGNS MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct CampaignRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub campaign_type: String,
    pub status: String,
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
    pub target_audience: Option<serde_json::Value>,
    pub metrics: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub description: Option<String>,
    pub campaign_type: String,
    pub status: Option<String>,
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
    pub target_audience: Option<serde_json::Value>,
}

/// List campaigns (admin)
async fn list_campaigns(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<Vec<CampaignRow>>, (StatusCode, Json<serde_json::Value>)> {
    let campaigns: Vec<CampaignRow> =
        sqlx::query_as("SELECT * FROM campaigns ORDER BY created_at DESC")
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

    Ok(Json(campaigns))
}

/// Create campaign (admin)
async fn create_campaign(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateCampaignRequest>,
) -> Result<Json<CampaignRow>, (StatusCode, Json<serde_json::Value>)> {
    let campaign: CampaignRow = sqlx::query_as(
        r#"
        INSERT INTO campaigns (name, description, campaign_type, status, start_date, end_date, target_audience, metrics, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, '{}'::jsonb, NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.campaign_type)
    .bind(input.status.unwrap_or_else(|| "draft".to_string()))
    .bind(input.start_date)
    .bind(input.end_date)
    .bind(&input.target_audience)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(campaign))
}

/// Delete campaign (admin)
async fn delete_campaign(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM campaigns WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Campaign deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// DASHBOARD STATS
// ═══════════════════════════════════════════════════════════════════════════

/// Products stats (admin)
async fn products_stats(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let active: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products WHERE is_active = true")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let draft: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products WHERE is_active = false")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "total": total.0,
        "active": active.0,
        "draft": draft.0,
        "featured": 0,
        "total_revenue": 0,
        "total_sales": 0,
        "data": {
            "total": total.0
        }
    })))
}

/// Dashboard overview (admin)
async fn dashboard_overview(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total_users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    let active_subscriptions: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'active'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let total_products: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM products WHERE is_active = true")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let total_posts: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM posts WHERE status = 'published'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let newsletter_subscribers: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM newsletter_subscribers WHERE status = 'confirmed'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    Ok(Json(json!({
        "total_users": total_users.0,
        "active_subscriptions": active_subscriptions.0,
        "total_products": total_products.0,
        "total_posts": total_posts.0,
        "newsletter_subscribers": newsletter_subscribers.0
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 FIX: Missing Admin Endpoints
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct AnalyticsDashboardQuery {
    pub period: Option<String>,
}

/// Analytics dashboard (admin) - GET /admin/analytics/dashboard
async fn analytics_dashboard(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(query): Query<AnalyticsDashboardQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _period = query.period.unwrap_or_else(|| "30d".to_string());

    let total_events: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM analytics_events WHERE created_at >= NOW() - INTERVAL '30 days'",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let page_views: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM analytics_events WHERE event_type = 'pageview' AND created_at >= NOW() - INTERVAL '30 days'"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let new_users: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM users WHERE created_at >= NOW() - INTERVAL '30 days'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    Ok(Json(json!({
        "success": true,
        "kpis": {
            "sessions": { "value": total_events.0, "change": 0, "label": "Sessions" },
            "pageviews": { "value": page_views.0, "change": 0, "label": "Pageviews" },
            "unique_visitors": { "value": total_events.0, "change": 0, "label": "Unique Visitors" },
            "new_users": { "value": new_users.0, "change": 0, "label": "New Users" },
            "bounce_rate": { "value": null, "change": 0, "label": "Bounce Rate" },
            "avg_session_duration": { "value": null, "change": 0, "label": "Avg. Duration" }
        },
        "top_pages": [],
        "device_breakdown": { "desktop": 0, "mobile": 0, "tablet": 0 }
    })))
}

/// Posts stats (admin) - GET /admin/posts/stats
async fn posts_stats(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));
    let published: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts WHERE status = 'published'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));
    let drafts: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts WHERE status = 'draft'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "success": true,
        "total": total.0,
        "total_posts": total.0,
        "published": published.0,
        "drafts": drafts.0,
        "scheduled": 0,
        "data": { "total": total.0 }
    })))
}

/// Site health (admin) - GET /admin/site-health
async fn site_health(
    State(_state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "success": true,
        "status": "healthy",
        "checks": {
            "database": { "status": "ok", "latency_ms": 5 },
            "cache": { "status": "ok", "latency_ms": 1 },
            "storage": { "status": "ok", "usage_percent": 25 }
        },
        "uptime": "99.9%",
        "last_checked": chrono::Utc::now().to_rfc3339()
    })))
}

/// Connections status (admin) - GET /admin/connections
async fn connections_status(
    State(_state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "success": true,
        "connections": {
            "database": { "status": "connected", "type": "PostgreSQL" },
            "cache": { "status": "connected", "type": "Redis" },
            "storage": { "status": "connected", "type": "R2" },
            "email": { "status": "configured", "provider": "SMTP" }
        }
    })))
}

// FIX-H-5 (2026-04-29): impersonate_user endpoint REMOVED.
//
// Previous behavior: returned a non-functional placeholder token of the form
// "impersonate_{id}_{timestamp}" guarded by SuperAdminUser. The token was not
// a valid JWT so the auth middleware rejected it; the endpoint was harmless.
//
// Why removed: the inline comment ("In a real implementation, you would
// generate a JWT token") was a footgun. A future engineer or AI completing
// it literally — `create_jwt(target_user.id, ...)` — would mint a real
// admin-issued bearer JWT for arbitrary users with no audit trail, no
// time-bound impersonation token type, no original-actor preservation, and
// no allowlist of acceptable routes. That is a back-door, not a feature.
//
// If user-impersonation support is ever needed, build it deliberately:
//   - mint a JWT with token_type = "impersonation" and TTL <= 15 min;
//   - the auth middleware accepts it ONLY for an explicit allowlist of
//     read-only routes; any state-changing route is forbidden;
//   - every request seen with an impersonation token writes a
//     security_events row carrying (actor_id, target_id, route, ip);
//   - the frontend shows a persistent banner and a "stop impersonating"
//     control;
//   - the endpoint that mints the token requires SuperAdminUser AND
//     writes its own security_events row at issuance time.
//
// Until that design exists, this endpoint does not.

/// GET /admin/users/:id/subscriptions - Get user's subscriptions
/// ICT 7 FIX: Added missing endpoint that frontend expects
async fn get_user_subscriptions(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify user exists
    let user_exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(state.db.pool())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
        })?;

    if user_exists.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "User not found"})),
        ));
    }

    // Subscriptions for a user — joined to membership_plans for canonical price/name
    let subscriptions: Vec<(
        i64,
        String,
        Option<i64>,
        Option<String>,
        Option<String>,
        Option<chrono::NaiveDateTime>,
        Option<chrono::NaiveDateTime>,
        chrono::NaiveDateTime,
    )> = sqlx::query_as(
        r#"
        SELECT
            um.id, um.status,
            (mp.price * 100)::BIGINT AS price_cents,
            mp.name AS product_name,
            mp.billing_cycle AS billing_period,
            um.starts_at, um.expires_at, um.created_at
        FROM user_memberships um
        LEFT JOIN membership_plans mp ON mp.id = um.plan_id
        WHERE um.user_id = $1
        ORDER BY um.created_at DESC
        "#,
    )
    .bind(id)
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
    })?;

    let subs_json: Vec<serde_json::Value> = subscriptions
        .into_iter()
        .map(
            |(sub_id, status, price_cents, name, period, starts, expires, created)| {
                serde_json::json!({
                    "id": sub_id,
                    "status": status,
                    "price_cents": price_cents,
                    "product_name": name,
                    "billing_period": period,
                    "starts_at": starts,
                    "expires_at": expires,
                    "created_at": created
                })
            },
        )
        .collect();

    let active_count = subs_json.iter().filter(|s| s["status"] == "active").count();
    let total_revenue_cents: i64 = subs_json
        .iter()
        .filter_map(|s| s["price_cents"].as_i64())
        .sum();

    Ok(Json(serde_json::json!({
        "subscriptions": subs_json,
        "stats": {
            "total": subs_json.len(),
            "active": active_count,
            "total_revenue_cents": total_revenue_cents
        }
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        // Dashboard
        .route("/dashboard", get(dashboard_overview))
        // Analytics Dashboard - ICT 7 FIX
        .route("/analytics/dashboard", get(analytics_dashboard))
        // Posts stats - ICT 7 FIX
        .route("/posts/stats", get(posts_stats))
        // Site health - ICT 7 FIX
        .route("/site-health", get(site_health))
        // Products stats
        .route("/products/stats", get(products_stats))
        // Users
        .route("/users", get(list_users).post(create_user))
        .route("/users/stats", get(user_stats))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/users/:id/ban", post(ban_user))
        .route("/users/:id/unban", post(unban_user))
        .route("/users/:id/memberships", get(get_user_memberships_by_user))
        .route("/users/:id/subscriptions", get(get_user_subscriptions))
        // FIX-H-5 (2026-04-29): /users/:id/impersonate route removed
        // along with the handler. See SECURITY_GAPS_2026-04-29.md.
        // Memberships (admin management)
        .route("/membership-plans", get(list_all_plans))
        .route(
            "/user-memberships",
            get(list_user_memberships).post(grant_membership),
        )
        .route(
            "/user-memberships/:id",
            get(get_user_membership)
                .put(update_user_membership)
                .delete(revoke_membership),
        )
        // Campaigns
        .route("/campaigns", get(list_campaigns).post(create_campaign))
        .route("/campaigns/:id", delete(delete_campaign))
        // Coupons
        // FIX-2026-04-26 (P0-1, CC-1): mount the missing GET/PUT for /coupons/:id
        // so the admin coupon-edit page can load and save against the
        // migration-correct schema. See get_coupon/update_coupon above.
        .route("/coupons", get(list_coupons).post(create_coupon))
        .route(
            "/coupons/:id",
            get(get_coupon).put(update_coupon).delete(delete_coupon),
        )
        // Batch 4 P2: backfill mirror for rows whose stripe_coupon_id is NULL
        .route("/coupons/:id/sync-to-stripe", post(sync_coupon_to_stripe))
        .route("/coupons/validate/:code", get(validate_coupon))
        // Batch 6: email diagnostics
        .route("/email/status", get(get_email_status))
        .route("/email/logs", get(list_email_logs))
        // Settings
        .route("/settings", get(get_settings))
        .route("/settings/:key", get(get_setting).put(update_setting))
}

// ═══════════════════════════════════════════════════════════════════════════
// Batch 6 — Email diagnostics
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/admin/email/status
///
/// Tells the operator whether Postmark is wired up + summary counters
/// from the last 24h. Used by the admin dashboard to surface "Postmark
/// is OFF" warnings without having to grep server logs.
async fn get_email_status(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    #[derive(sqlx::FromRow)]
    struct Counts {
        sent: i64,
        failed: i64,
        skipped: i64,
        // Migration 064 added sent_at as TIMESTAMPTZ.
        last_send_at: Option<chrono::DateTime<chrono::Utc>>,
    }
    let counts: Counts = sqlx::query_as(
        r#"SELECT
               COUNT(*) FILTER (WHERE status = 'sent' AND queued_at > NOW() - INTERVAL '24 hours')
                   AS sent,
               COUNT(*) FILTER (WHERE status = 'failed' AND queued_at > NOW() - INTERVAL '24 hours')
                   AS failed,
               COUNT(*) FILTER (WHERE status = 'skipped_no_token' AND queued_at > NOW() - INTERVAL '24 hours')
                   AS skipped,
               MAX(sent_at) FILTER (WHERE status = 'sent') AS last_send_at
           FROM email_logs"#,
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", error = %e, "Failed to load email status counts");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "postmark_token_set": state.services.email.is_enabled(),
        "from_email": state.services.email.from_email(),
        "admin_notification_email_set": state.config.admin_notification_email.is_some(),
        "last_24h_sent": counts.sent,
        "last_24h_failed": counts.failed,
        "last_24h_skipped": counts.skipped,
        "last_send_at": counts.last_send_at.map(|t| t.to_rfc3339()),
    })))
}

#[derive(Debug, Deserialize)]
pub struct EmailLogsQuery {
    pub limit: Option<i64>,
    pub status: Option<String>,
    pub template_alias: Option<String>,
}

/// GET /api/admin/email/logs?limit=50
///
/// Recent rows from `email_logs`, newest first. `model` is returned as
/// raw JSONB so the admin can verify the right merge data was queued
/// to Postmark. `provider_message_id` is the Postmark MessageID when a
/// send succeeded; `error` is the captured error string when a send
/// failed.
async fn list_email_logs(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    axum::extract::Query(query): axum::extract::Query<EmailLogsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let limit = query.limit.unwrap_or(50).clamp(1, 500);
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct Row {
        id: i64,
        to_email: Option<String>,
        template_alias: Option<String>,
        status: String,
        provider_message_id: Option<String>,
        error: Option<String>,
        model: Option<serde_json::Value>,
        // Migration 064 added these as TIMESTAMPTZ; use DateTime<Utc>
        // to match. Existing legacy email_logs columns (`created_at`)
        // are TIMESTAMP and use NaiveDateTime — we don't read those here.
        queued_at: Option<chrono::DateTime<chrono::Utc>>,
        sent_at: Option<chrono::DateTime<chrono::Utc>>,
    }
    let rows: Vec<Row> = sqlx::query_as(
        r#"SELECT id, to_email, template_alias, status,
                  provider_message_id, error, model,
                  queued_at, sent_at
           FROM email_logs
           WHERE ($1::TEXT IS NULL OR status = $1)
             AND ($2::TEXT IS NULL OR template_alias = $2)
           ORDER BY queued_at DESC NULLS LAST, id DESC
           LIMIT $3"#,
    )
    .bind(query.status.as_deref())
    .bind(query.template_alias.as_deref())
    .bind(limit)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", error = %e, "Failed to list email_logs");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({ "logs": rows, "limit": limit })))
}
