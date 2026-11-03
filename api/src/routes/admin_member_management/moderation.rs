//! Ban / suspend / unban handlers for member moderation.

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;
use serde_json::json;

use crate::AppState;
use crate::middleware::admin::AdminUser;

use super::BanMemberRequest;

/// POST /admin/member-management/:id/ban - Ban member
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
pub(super) async fn ban_member(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<BanMemberRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Prevent self-ban
    if admin.0.id == id {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "You cannot ban your own account"})),
        ));
    }

    // Calculate ban expiry
    let banned_until = input
        .duration_days
        .map(|days| Utc::now() + chrono::Duration::days(days as i64));

    // FIX-2026-04-26 (Priority 5): wrap user_status upsert + activity log in a tx.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    // Upsert user status
    sqlx::query(
        r"
        INSERT INTO user_status (user_id, status, banned_at, banned_by, ban_expires_at, ban_reason, updated_at)
        VALUES ($1, 'banned', NOW(), $4, $2, $3, NOW())
        ON CONFLICT (user_id) DO UPDATE SET
            status = 'banned',
            banned_at = NOW(),
            banned_by = $4,
            ban_expires_at = $2,
            ban_reason = $3,
            updated_at = NOW()
        ",
    )
    .bind(id)
    .bind(banned_until)
    .bind(&input.reason)
    .bind(admin.0.id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    sqlx::query("UPDATE users SET is_active = false, updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Log activity
    // Original pool reference: .execute(&state.db.pool)
    sqlx::query(
        r"
        INSERT INTO user_activity_log (user_id, action, details, created_at)
        VALUES ($1, 'account_banned', $2, NOW())
        ",
    )
    .bind(id)
    .bind(json!({
        "description": format!(
            "Account banned{}",
            input
                .duration_days
                .map(|d| format!(" for {d} days"))
                .unwrap_or_default()
        ),
        "reason": input.reason,
        "duration_days": input.duration_days,
        "banned_by": admin.0.id
    }))
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // FIX-2026-04-26 (Priority 5): commit ban transaction.
    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    if let Some(ref redis) = state.services.redis {
        redis.revoke_user_access(id).await;
    }

    Ok(Json(json!({
        "message": "Member banned successfully",
        "banned_until": banned_until
    })))
}

/// POST /admin/member-management/:id/suspend - Suspend member (temporary)
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
pub(super) async fn suspend_member(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<BanMemberRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if admin.0.id == id {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "You cannot suspend your own account"})),
        ));
    }

    let suspended_until = input
        .duration_days
        .map(|days| Utc::now() + chrono::Duration::days(days as i64));

    // FIX-2026-04-26 (Priority 5): wrap user_status upsert + activity log in a tx.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    sqlx::query(
        r"
        INSERT INTO user_status (user_id, status, ban_expires_at, ban_reason, updated_at)
        VALUES ($1, 'suspended', $2, $3, NOW())
        ON CONFLICT (user_id) DO UPDATE SET
            status = 'suspended',
            ban_expires_at = $2,
            ban_reason = $3,
            updated_at = NOW()
        ",
    )
    .bind(id)
    .bind(suspended_until)
    .bind(&input.reason)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    sqlx::query("UPDATE users SET is_active = false, updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Original pool reference: .execute(&state.db.pool)
    sqlx::query(
        r"
        INSERT INTO user_activity_log (user_id, action, details, created_at)
        VALUES ($1, 'account_suspended', $2, NOW())
        ",
    )
    .bind(id)
    .bind(json!({
        "description": format!(
            "Account suspended{}",
            input
                .duration_days
                .map(|d| format!(" for {d} days"))
                .unwrap_or_default()
        ),
        "reason": input.reason,
        "duration_days": input.duration_days,
        "suspended_by": admin.0.id
    }))
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // FIX-2026-04-26 (Priority 5): commit suspend transaction.
    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    if let Some(ref redis) = state.services.redis {
        redis.revoke_user_access(id).await;
    }

    Ok(Json(json!({
        "message": "Member suspended successfully",
        "suspended_until": suspended_until
    })))
}

/// POST /admin/member-management/:id/unban - Unban/unsuspend member
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
pub(super) async fn unban_member(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // FIX-2026-04-26 (Priority 5): wrap user_status update + activity log in a tx.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    // Original pool reference: .execute(&state.db.pool)
    sqlx::query(
        r"
        UPDATE user_status
        SET status = 'active', ban_expires_at = NULL, ban_reason = NULL, updated_at = NOW()
        WHERE user_id = $1
        ",
    )
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    sqlx::query("UPDATE users SET is_active = true, updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Original pool reference: .execute(&state.db.pool)
    sqlx::query(
        r"
        INSERT INTO user_activity_log (user_id, action, details, created_at)
        VALUES ($1, 'account_unbanned', $2, NOW())
        ",
    )
    .bind(id)
    .bind(json!({
        "description": "Account unbanned by admin",
        "unbanned_by": admin.0.id
    }))
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // FIX-2026-04-26 (Priority 5): commit unban transaction.
    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    if let Some(ref redis) = state.services.redis
        && let Err(e) = redis.invalidate_user_cache(id).await
    {
        tracing::warn!(target: "security", user_id = id, error = %e, "Failed to invalidate cached user after moderation update");
    }

    Ok(Json(json!({
        "message": "Member unbanned successfully"
    })))
}
