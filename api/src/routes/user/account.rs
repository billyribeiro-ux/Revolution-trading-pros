//! Account deactivation (self-service).
//!
//! R26-B split: handler moved verbatim from `routes/user.rs`. The
//! single-transaction invariant (`user_activity_log`,
//! `user_memberships`, and `users` commit atomically) is preserved.
//! Leaving a user marked `deleted` while their subscriptions are
//! still `active` would be a billing-fraud-class bug.

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::json;

use crate::{models::User, AppState};

/// Deactivate account request
#[derive(Debug, Deserialize)]
pub(super) struct DeactivateAccountRequest {
    password: String,
    reason: Option<String>,
    #[serde(default)]
    delete_data: bool,
}

/// Self-service account deactivation
/// POST /api/user/deactivate
/// User must confirm with their password
pub(super) async fn deactivate_account(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<DeactivateAccountRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify password — surface a 500 on hash-format / library failures rather
    // than masquerading them as a wrong-password 400, but never leak the
    // internal error to the caller.
    let password_valid = crate::utils::verify_password(&input.password, &user.password_hash)
        .map_err(|e| {
            tracing::error!(user_id = user.id, "Password verification error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Password verification failed" })),
            )
        })?;
    if !password_valid {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Password is incorrect",
                "field": "password"
            })),
        ));
    }

    // ICT 7 SAFETY: deactivation touches user_activity_log + (optionally)
    // user_memberships + users. All of these must commit atomically — leaving a
    // user marked 'deleted' while their subscriptions are still 'active' is a
    // billing-fraud-class bug.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!("tx start (deactivate_account): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Log the deactivation reason
    if let Some(reason) = &input.reason {
        sqlx::query(
            r"
            INSERT INTO user_activity_log (user_id, action, description, created_at)
            VALUES ($1, 'account_deactivation_requested', $2, NOW())
            ",
        )
        .bind(user.id)
        .bind(reason)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert user_activity_log: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to record deactivation"})),
            )
        })?;
    }

    if input.delete_data {
        // Hard delete - remove all user data (GDPR compliance)
        // First, cancel any active subscriptions
        sqlx::query("UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW() WHERE user_id = $1 AND status IN ('active', 'trialing')")
            .bind(user.id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to cancel user_memberships during deletion: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to cancel subscriptions"})),
                )
            })?;

        // Soft delete the user (keep for audit trail, anonymize PII)
        sqlx::query(
            r"
            UPDATE users SET
                email = CONCAT('deleted_', id, '@deactivated.local'),
                name = 'Deleted User',
                first_name = NULL,
                last_name = NULL,
                avatar_url = NULL,
                password_hash = 'DEACTIVATED',
                status = 'deleted',
                deleted_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            ",
        )
        .bind(user.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

        tx.commit().await.map_err(|e| {
            tracing::error!("tx commit (deactivate_account, delete): {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

        tracing::info!(
            target: "security_audit",
            event = "account_deleted",
            user_id = %user.id,
            "ICT 7 AUDIT: User account permanently deleted"
        );

        Ok(Json(json!({
            "success": true,
            "message": "Your account has been permanently deleted. All personal data has been removed."
        })))
    } else {
        // Soft deactivation - keep data but disable account
        sqlx::query(
            r"
            UPDATE users SET
                status = 'deactivated',
                updated_at = NOW()
            WHERE id = $1
            ",
        )
        .bind(user.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

        tx.commit().await.map_err(|e| {
            tracing::error!("tx commit (deactivate_account, soft): {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

        tracing::info!(
            target: "security_audit",
            event = "account_deactivated",
            user_id = %user.id,
            "ICT 7 AUDIT: User account deactivated"
        );

        Ok(Json(json!({
            "success": true,
            "message": "Your account has been deactivated. You can reactivate it by logging in again."
        })))
    }
}
