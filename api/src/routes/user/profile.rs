//! User profile handlers — read + update (with password change &
//! email verification).
//!
//! R26-B split: handlers moved verbatim from `routes/user.rs`. The
//! single-transaction invariant (users + email_verification_tokens
//! commit atomically; external email send happens AFTER commit) is
//! preserved.

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::json;

use crate::{models::User, AppState};

/// Get current user profile
/// Matches frontend expectation: GET /api/user/profile
pub(super) async fn get_profile(user: User) -> Json<crate::models::UserResponse> {
    Json(user.into())
}

/// Update user profile request - ICT 7 Principal Engineer Grade
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UpdateProfileRequest {
    #[serde(alias = "first_name")]
    first_name: Option<String>,
    #[serde(alias = "last_name")]
    last_name: Option<String>,
    display_name: Option<String>,
    email: Option<String>,
    current_password: Option<String>,
    new_password: Option<String>,
}

/// Update user profile
/// PUT /api/user/profile
/// ICT 7 Principal Engineer Grade - Full profile update with password change support
pub(super) async fn update_profile(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<UpdateProfileRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Pre-tx: validate password change inputs (no DB writes here so we can fail
    // fast without opening a transaction).
    let new_password_hash: Option<String> = if let (Some(current_password), Some(new_password)) =
        (&input.current_password, &input.new_password)
    {
        // Verify current password — propagate library / hash-format failures
        // as 500 so they get logged and alerted, instead of silently treating
        // them as a wrong-password 400.
        let password_valid = crate::utils::verify_password(current_password, &user.password_hash)
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
                    "error": "Current password is incorrect",
                    "field": "currentPassword"
                })),
            ));
        }

        // Validate new password strength
        if let Err(password_error) = crate::utils::validate_password(new_password) {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "error": password_error,
                    "field": "newPassword"
                })),
            ));
        }

        // Hash new password
        let hash = crate::utils::hash_password(new_password).map_err(|e| {
            tracing::error!("Password hashing error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to process password"})),
            )
        })?;
        Some(hash)
    } else {
        None
    };

    // Check if email is being changed
    let email_changed = input.email.as_ref().is_some_and(|e| e != &user.email);

    // Build display_name from first_name + last_name if not provided
    let display_name =
        input
            .display_name
            .clone()
            .or_else(|| match (&input.first_name, &input.last_name) {
                (Some(f), Some(l)) => Some(format!("{f} {l}")),
                (Some(f), None) => Some(f.clone()),
                (None, Some(l)) => Some(l.clone()),
                (None, None) => None,
            });

    // Pre-generate verification token (so the raw_token is available outside the
    // tx for the post-commit email send).
    let verification_token = if email_changed && input.email.is_some() {
        Some(crate::utils::generate_verification_token())
    } else {
        None
    };

    // ICT 7 SAFETY: profile updates can touch users (password, profile fields,
    // email_verified_at) AND email_verification_tokens in the same handler. Wrap
    // them in a single tx so a partial failure cannot leave a user with their
    // new email but no verification token row (locking them out) or vice versa.
    // External email send happens AFTER commit — never hold a DB tx across HTTP.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!("tx start (update_profile): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Update password if requested
    if let Some(ref hash) = new_password_hash {
        sqlx::query("UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2")
            .bind(hash)
            .bind(user.id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;
    }

    // Update profile fields in database
    sqlx::query(
        r"UPDATE users SET
            first_name = COALESCE($1, first_name),
            last_name = COALESCE($2, last_name),
            name = COALESCE($3, name),
            email = COALESCE($4, email),
            updated_at = NOW()
        WHERE id = $5",
    )
    .bind(&input.first_name)
    .bind(&input.last_name)
    .bind(&display_name)
    .bind(&input.email)
    .bind(user.id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        // Check for unique constraint violation on email
        let err_str = e.to_string();
        if err_str.contains("unique") || err_str.contains("duplicate") {
            return (
                StatusCode::CONFLICT,
                Json(json!({
                    "error": "Email address is already in use",
                    "field": "email"
                })),
            );
        }
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // If email changed, mark unverified and persist a verification token in the
    // same tx as the email update.
    if let Some((_, ref hashed_token)) = verification_token {
        sqlx::query("UPDATE users SET email_verified_at = NULL WHERE id = $1")
            .bind(user.id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to clear email_verified_at: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to update verification state"})),
                )
            })?;

        sqlx::query(
            r"
            INSERT INTO email_verification_tokens (user_id, token, expires_at)
            VALUES ($1, $2, NOW() + INTERVAL '24 hours')
            ON CONFLICT (user_id) DO UPDATE SET token = $2, expires_at = NOW() + INTERVAL '24 hours'
            ",
        )
        .bind(user.id)
        .bind(hashed_token)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to upsert email_verification_token: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to issue verification token"})),
            )
        })?;
    }

    tx.commit().await.map_err(|e| {
        tracing::error!("tx commit (update_profile): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    if new_password_hash.is_some() {
        tracing::info!(
            target: "security_audit",
            event = "password_changed",
            user_id = %user.id,
            "ICT 7 AUDIT: User password changed successfully"
        );
    }

    // Post-commit: send verification email (external HTTP, never inside a tx).
    if email_changed {
        if let (Some(new_email), Some((raw_token, _))) = (&input.email, verification_token) {
            // Send verification email
            {
                let email_service = &state.services.email;
                let name = display_name.as_deref().unwrap_or(&user.name);
                email_service
                    .send_verification_email(new_email, name, &raw_token)
                    .await
                    .ok();
            }

            tracing::info!(
                target: "security_audit",
                event = "email_change_verification_sent",
                user_id = %user.id,
                new_email = %new_email,
                "ICT 7 AUDIT: Email change verification sent"
            );

            return Ok(Json(json!({
                "success": true,
                "message": "Profile updated. Please verify your new email address.",
                "email_verification_required": true
            })));
        }
    }

    Ok(Json(json!({
        "success": true,
        "message": "Profile updated successfully"
    })))
}
