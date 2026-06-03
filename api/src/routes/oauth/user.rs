//! OAuth user lifecycle — create/link/login and the auth-response mint.
//!
//! Centralizes the three database flows shared by both Google and Apple
//! callbacks:
//!
//! 1. **OAuth-ID match** — find an existing user by `google_id` / `apple_id`,
//!    log a `login` audit row, return.
//! 2. **Email match** — link the OAuth provider ID to the existing email
//!    account (`oauth_linked_at = COALESCE(..., NOW())`), log a `link` audit
//!    row, refresh and return.
//! 3. **New user** — create a fresh row with a throw-away password hash,
//!    log a `register` audit row, return.
//!
//! The token-mint path (`create_oauth_auth_response`) reads the user's live
//! Redis epoch and fails CLOSED on Redis read errors per NF-3b (P1):
//! previously OAuth tokens were minted with `token_version == 0`, bypassing
//! revocation. DO NOT relax the fail-closed semantics without a security
//! sign-off.

use axum::{http::StatusCode, Json};

use super::crypto::generate_random_string;
use super::types::OAuthProvider;
use crate::{
    models::{AuthResponse, User},
    utils::{create_jwt_versioned, create_refresh_token_versioned, generate_session_id},
    AppState,
};

/// Create or get existing user from OAuth data
#[allow(clippy::too_many_arguments)]
pub(super) async fn create_or_get_oauth_user(
    state: &AppState,
    provider: OAuthProvider,
    provider_id: &str,
    email: &str,
    name: Option<String>,
    avatar_url: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<User, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7: First, check if user exists with this OAuth provider ID
    let provider_column = match provider {
        OAuthProvider::Google => "google_id",
        OAuthProvider::Apple => "apple_id",
    };

    let existing_oauth_user: Option<User> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at
         FROM users WHERE {provider_column} = $1"
    )))
    .bind(provider_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database error"})),
        )
    })?;

    if let Some(user) = existing_oauth_user {
        // ICT 7 AUDIT: Log OAuth login
        log_oauth_action(
            state,
            Some(user.id),
            provider,
            "login",
            provider_id,
            ip_address.as_deref(),
            user_agent.as_deref(),
        )
        .await;
        return Ok(user);
    }

    // ICT 7: Check if user exists with this email
    let existing_email_user: Option<User> = sqlx::query_as(
        "SELECT id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at
         FROM users WHERE email = $1"
    )
    .bind(email)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database error"})),
        )
    })?;

    if let Some(user) = existing_email_user {
        // ICT 7: Link OAuth to existing account
        let update_query = format!(
            "UPDATE users SET {provider_column} = $1, oauth_linked_at = COALESCE(oauth_linked_at, NOW()), updated_at = NOW() WHERE id = $2"
        );

        sqlx::query(sqlx::AssertSqlSafe(update_query.as_str()))
            .bind(provider_id)
            .bind(user.id)
            .execute(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to link OAuth: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Failed to link OAuth account"})),
                )
            })?;

        // ICT 7 AUDIT: Log OAuth link
        log_oauth_action(
            state,
            Some(user.id),
            provider,
            "link",
            provider_id,
            ip_address.as_deref(),
            user_agent.as_deref(),
        )
        .await;

        tracing::info!(
            target: "security_audit",
            event = "oauth_account_linked",
            user_id = %user.id,
            provider = %provider,
            "ICT 7 AUDIT: OAuth account linked to existing user"
        );

        // Refresh user data
        let updated_user: User = sqlx::query_as(
            "SELECT id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at
             FROM users WHERE id = $1"
        )
        .bind(user.id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Database error"})),
            )
        })?;

        return Ok(updated_user);
    }

    // ICT 7: Create new user with OAuth
    let user_name = name.unwrap_or_else(|| email.split('@').next().unwrap_or("User").to_string());

    // Generate a random password hash for OAuth users (they won't use it)
    let random_password = generate_random_string(32);
    let password_hash = crate::utils::hash_password(&random_password).map_err(|e| {
        tracing::error!("Password hashing error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Account creation failed"})),
        )
    })?;

    let insert_query = format!(
        r"
        INSERT INTO users (
            email, password_hash, name, role, email_verified_at, avatar_url,
            {provider_column}, oauth_provider, oauth_linked_at, created_at, updated_at
        )
        VALUES ($1, $2, $3, 'user', NOW(), $4, $5, $6, NOW(), NOW(), NOW())
        RETURNING id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at
        "
    );

    let new_user: User = sqlx::query_as(sqlx::AssertSqlSafe(insert_query.as_str()))
        .bind(email)
        .bind(&password_hash)
        .bind(&user_name)
        .bind(&avatar_url)
        .bind(provider_id)
        .bind(provider.to_string())
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create OAuth user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to create account"})),
            )
        })?;

    // ICT 7 AUDIT: Log OAuth registration
    log_oauth_action(
        state,
        Some(new_user.id),
        provider,
        "register",
        provider_id,
        ip_address.as_deref(),
        user_agent.as_deref(),
    )
    .await;

    tracing::info!(
        target: "security_audit",
        event = "oauth_user_created",
        user_id = %new_user.id,
        provider = %provider,
        "ICT 7 AUDIT: New user created via OAuth"
    );

    Ok(new_user)
}

/// Log OAuth action to audit table
async fn log_oauth_action(
    state: &AppState,
    user_id: Option<i64>,
    provider: OAuthProvider,
    action: &str,
    provider_user_id: &str,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) {
    let _ = sqlx::query(
        r"
        INSERT INTO oauth_audit_log (user_id, provider, action, provider_user_id, ip_address, user_agent)
        VALUES ($1, $2, $3, $4, $5, $6)
        ",
    )
    .bind(user_id)
    .bind(provider.to_string())
    .bind(action)
    .bind(provider_user_id)
    .bind(ip_address)
    .bind(user_agent)
    .execute(&state.db.pool)
    .await;
}

/// Create auth response for OAuth user
///
/// NF-3b (P1): the OAuth path previously minted tokens via the
/// `token_version == 0` shims, bypassing the P1-2 revocation epoch — a
/// stolen OAuth token survived logout-all/reset/ban, and an OAuth login
/// after any epoch bump was silently un-revocable. Mint with the user's
/// live epoch (fail-closed if the epoch can't be read), exactly like the
/// password-login path in `auth.rs`.
pub(super) async fn create_oauth_auth_response(
    user: User,
    state: &AppState,
) -> Result<AuthResponse, (StatusCode, Json<serde_json::Value>)> {
    let token_version = match state.services.redis.as_ref() {
        Some(redis) => redis.get_token_version(user.id).await.map_err(|e| {
            tracing::error!(
                target: "security_audit",
                event = "token_version_read_failed_fail_closed",
                user_id = %user.id,
                error = %e,
                "Could not read token epoch; refusing to mint OAuth token (fail-closed)"
            );
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "Service temporarily unavailable, please try again"})),
            )
        })?,
        None => 0,
    };

    // Create access token
    let token = create_jwt_versioned(
        user.id,
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
        token_version,
    )
    .map_err(|e| {
        tracing::error!("JWT creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Token creation failed"})),
        )
    })?;

    // Create refresh token
    let refresh_token =
        create_refresh_token_versioned(user.id, &state.config.jwt_secret, token_version).map_err(
            |e| {
                tracing::error!("Refresh token creation error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Refresh token creation failed"})),
                )
            },
        )?;

    // Generate session ID
    let session_id = generate_session_id();

    Ok(AuthResponse {
        token: token.clone(),
        access_token: token,
        refresh_token,
        session_id,
        user: user.into(),
        expires_in: state.config.jwt_expires_in * 3600,
    })
}
