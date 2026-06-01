//! Ban / suspend / unban handlers for member moderation.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

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
        .map(|days| Utc::now().naive_utc() + chrono::Duration::days(days as i64));

    // FIX-2026-04-26 (Priority 5): wrap user_status upsert + activity log in a tx.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    // Upsert user status
    // Original pool reference: .execute(&state.db.pool)
    sqlx::query(
        r"
        INSERT INTO user_status (user_id, status, banned_until, ban_reason, updated_at)
        VALUES ($1, 'banned', $2, $3, NOW())
        ON CONFLICT (user_id) DO UPDATE SET
            status = 'banned',
            banned_until = $2,
            ban_reason = $3,
            updated_at = NOW()
        ",
    )
    .bind(id)
    .bind(banned_until)
    .bind(&input.reason)
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
    let _ = sqlx::query(
        r"
        INSERT INTO user_activity_log (user_id, action, description, metadata, created_at)
        VALUES ($1, 'account_banned', $2, $3, NOW())
        ",
    )
    .bind(id)
    .bind(format!(
        "Account banned{}",
        input
            .duration_days
            .map(|d| format!(" for {d} days"))
            .unwrap_or_default()
    ))
    .bind(json!({
        "reason": input.reason,
        "duration_days": input.duration_days,
        "banned_by": admin.0.id
    }))
    .execute(&mut *tx)
    .await;

    // FIX-2026-04-26 (Priority 5): commit ban transaction.
    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

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
        .map(|days| Utc::now().naive_utc() + chrono::Duration::days(days as i64));

    // FIX-2026-04-26 (Priority 5): wrap user_status upsert + activity log in a tx.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    // Original pool reference: .execute(&state.db.pool)
    sqlx::query(
        r"
        INSERT INTO user_status (user_id, status, banned_until, ban_reason, updated_at)
        VALUES ($1, 'suspended', $2, $3, NOW())
        ON CONFLICT (user_id) DO UPDATE SET
            status = 'suspended',
            banned_until = $2,
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

    // Original pool reference: .execute(&state.db.pool)
    let _ = sqlx::query(
        r"
        INSERT INTO user_activity_log (user_id, action, description, metadata, created_at)
        VALUES ($1, 'account_suspended', $2, $3, NOW())
        ",
    )
    .bind(id)
    .bind(format!(
        "Account suspended{}",
        input
            .duration_days
            .map(|d| format!(" for {d} days"))
            .unwrap_or_default()
    ))
    .bind(json!({
        "reason": input.reason,
        "duration_days": input.duration_days,
        "suspended_by": admin.0.id
    }))
    .execute(&mut *tx)
    .await;

    // FIX-2026-04-26 (Priority 5): commit suspend transaction.
    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

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
        SET status = 'active', banned_until = NULL, ban_reason = NULL, updated_at = NOW()
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

    // Original pool reference: .execute(&state.db.pool)
    let _ = sqlx::query(
        r"
        INSERT INTO user_activity_log (user_id, action, description, metadata, created_at)
        VALUES ($1, 'account_unbanned', 'Account unbanned by admin', $2, NOW())
        ",
    )
    .bind(id)
    .bind(json!({"unbanned_by": admin.0.id}))
    .execute(&mut *tx)
    .await;

    // FIX-2026-04-26 (Priority 5): commit unban transaction.
    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "message": "Member unbanned successfully"
    })))
}
