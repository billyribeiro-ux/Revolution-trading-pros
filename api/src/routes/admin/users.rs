//! Admin user management (admin staff) — extracted from the original
//! `routes/admin.rs` as part of the R6-B split (2026-05-20).
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
pub(super) async fn list_users(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(query): Query<UserListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    // ICT 11+ SECURITY: Use NULL-safe parameterized queries
    // This approach is SQL injection proof and compile-time verified
    let search_pattern: Option<String> = query.search.as_ref().map(|s| format!("%{s}%"));

    let users: Vec<AdminUserRow> = sqlx::query_as(
        r"
        SELECT id, name, email, role, is_active, email_verified_at, last_login_at, created_at, updated_at
        FROM users
        WHERE ($1::text IS NULL OR role = $1)
          AND ($2::boolean IS NULL OR is_active = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR email ILIKE $3)
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        "
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
        r"
        SELECT COUNT(*)
        FROM users
        WHERE ($1::text IS NULL OR role = $1)
          AND ($2::boolean IS NULL OR is_active = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR email ILIKE $3)
        ",
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
pub(super) async fn get_user(
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
pub(super) async fn create_user(
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
        r"
        INSERT INTO users (name, email, password_hash, role, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, true, NOW(), NOW())
        RETURNING id, name, email, role, is_active, email_verified_at, last_login_at, created_at, updated_at
        "
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
pub(super) async fn update_user(
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
        set_clauses.push(format!("name = ${param_count}"));
    }
    if input.email.is_some() {
        param_count += 1;
        set_clauses.push(format!("email = ${param_count}"));
    }
    if input.role.is_some() {
        param_count += 1;
        set_clauses.push(format!("role = ${param_count}"));
    }
    if input.is_active.is_some() {
        param_count += 1;
        set_clauses.push(format!("is_active = ${param_count}"));
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

    let mut query_builder =
        sqlx::query_as::<_, AdminUserRow>(sqlx::AssertSqlSafe(sql.as_str())).bind(id);

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
            r"INSERT INTO security_events (user_id, event_type, event_category, severity, details)
               VALUES ($1, 'role_change', 'access_control', 'high', $2)",
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
pub(super) async fn delete_user(
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
pub(super) async fn ban_user(
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
        // SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2) — required companion to
        // commit 6f5c5a9 (per-user token-version revocation). Deleting
        // `session:*` keys alone never touched the stateless access/refresh
        // JWTs: that was the root bug — a just-banned user's *stolen* access
        // token stayed valid for its full TTL (≤1h) and the refresh token
        // until rotation (≤7d), because the C-1 `is_active` guard + the
        // extractor only re-check on the next DB-backed fetch, not on a token
        // served from the Redis user-cache hot path. Bumping the token epoch
        // strands every previously-issued token for this user the instant the
        // extractor/refresh handler re-reads the epoch. Best-effort, exactly
        // as `auth.rs::logout_all` / `reset_password` do it: a Redis fault is
        // logged but does NOT fail the ban (DB ban is already persisted and
        // the extractor's read fails closed on a Redis fault at validate time).
        if let Err(e) = redis.bump_token_version(id).await {
            tracing::error!(
                target: "security_audit",
                event = "ban_token_version_bump_failed",
                user_id = %id,
                error = %e,
                "Could not bump token epoch on ban (DB ban still effective; extractor fails closed on Redis error)"
            );
        }
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
pub(super) async fn unban_user(
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
pub(super) async fn user_stats(
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
