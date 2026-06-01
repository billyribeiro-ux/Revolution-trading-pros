//! Sign in with Apple — init + callback handlers.
//!
//! Security contract:
//! - `state` parameter is generated server-side, persisted to `oauth_states`,
//!   and rejected on mismatch / expiry / re-use (`used_at IS NULL` filter).
//! - `nonce` is generated server-side, stored in the same `oauth_states`
//!   row, and verified against the ID token's `nonce` claim in
//!   `apple_jwt::validate_apple_id_token` (SHA-256 + URL-safe base64).
//! - Apple ID token is cryptographically validated against Apple's live
//!   JWKS (RS256 signature, `iss = https://appleid.apple.com`, `aud =
//!   APPLE_CLIENT_ID`).
//! - Apple callback is `form_post` (response_mode=form_post) so the
//!   callback handler is POST, not GET.
//! - Tokens are returned via httpOnly cookies on the redirect Response
//!   (FIX-C-1, 2026-04-29) — never in the URL.

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::apple_jwt::validate_apple_id_token;
use super::crypto::generate_random_string;
use super::session::oauth_callback_response_with_cookies;
use super::types::{AppleCallbackBody, OAuthProvider};
use super::user::{create_oauth_auth_response, create_or_get_oauth_user};
use crate::{utils::generate_session_id, AppState};

/// Apple user info (only provided on first auth)
#[derive(Debug, Deserialize)]
struct AppleUserInfo {
    name: Option<AppleName>,
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AppleName {
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    #[serde(rename = "lastName")]
    last_name: Option<String>,
}

/// Initiate Apple Sign-In flow
/// GET /api/auth/apple
pub(super) async fn apple_init(
    State(state): State<AppState>,
) -> Result<Redirect, (StatusCode, Json<serde_json::Value>)> {
    let apple_client_id = std::env::var("APPLE_CLIENT_ID").map_err(|_| {
        tracing::error!("APPLE_CLIENT_ID not configured");
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Apple Sign-In not configured"})),
        )
    })?;

    // Generate state for CSRF protection
    let oauth_state = generate_random_string(32);

    // Generate nonce for ID token validation
    let nonce = generate_random_string(32);
    let nonce_hash = {
        let mut hasher = Sha256::new();
        hasher.update(nonce.as_bytes());
        let hash = hasher.finalize();
        URL_SAFE_NO_PAD.encode(hash)
    };

    // Store state and nonce in database
    sqlx::query(
        r"
        INSERT INTO oauth_states (state, provider, nonce, expires_at)
        VALUES ($1, 'apple', $2, NOW() + INTERVAL '10 minutes')
        ",
    )
    .bind(&oauth_state)
    .bind(&nonce)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to store OAuth state: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "OAuth initialization failed"})),
        )
    })?;

    // Build Apple authorization URL
    let redirect_uri = format!("{}/api/auth/apple/callback", state.config.app_url);
    let authorization_url = format!(
        "https://appleid.apple.com/auth/authorize?\
        client_id={}&\
        redirect_uri={}&\
        response_type=code%20id_token&\
        response_mode=form_post&\
        scope=name%20email&\
        state={}&\
        nonce={}",
        urlencoding::encode(&apple_client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&oauth_state),
        urlencoding::encode(&nonce_hash),
    );

    tracing::info!(
        target: "security_audit",
        event = "apple_oauth_initiated",
        state = %oauth_state,
        "ICT 7 AUDIT: Apple Sign-In flow initiated"
    );

    Ok(Redirect::to(&authorization_url))
}

/// Handle Apple Sign-In callback
/// POST /api/auth/apple/callback (Apple uses form_post response mode)
pub(super) async fn apple_callback(
    State(state): State<AppState>,
    axum::Form(body): axum::Form<AppleCallbackBody>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // FIX-C-1 (2026-04-29): see google_callback for rationale.
    // Check for errors from Apple
    if let Some(error) = body.error {
        tracing::warn!(
            target: "security_audit",
            event = "apple_oauth_error",
            error = %error,
            "Apple Sign-In error"
        );
        return Ok(Redirect::to(&format!(
            "{}/login?error={}",
            state.config.app_url,
            urlencoding::encode(&format!("Apple sign-in failed: {error}"))
        ))
        .into_response());
    }

    // Validate state parameter
    let oauth_state = body.state.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing state parameter"})),
        )
    })?;

    // Get and validate stored state
    let state_record: Option<(String,)> = sqlx::query_as(
        "SELECT nonce FROM oauth_states WHERE state = $1 AND provider = 'apple' AND expires_at > NOW() AND used_at IS NULL"
    )
    .bind(&oauth_state)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database error"})),
        )
    })?;

    let stored_nonce = state_record
        .ok_or_else(|| {
            tracing::warn!(
                target: "security_audit",
                event = "apple_oauth_invalid_state",
                state = %oauth_state,
                "Invalid or expired OAuth state"
            );
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Invalid or expired state"})),
            )
        })?
        .0;

    // Mark state as used
    sqlx::query("UPDATE oauth_states SET used_at = NOW() WHERE state = $1")
        .bind(&oauth_state)
        .execute(&state.db.pool)
        .await
        .ok();

    // Get ID token (Apple provides it directly in the callback)
    let id_token = body.id_token.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing ID token"})),
        )
    })?;

    // ICT 7 SECURITY: Validate Apple ID token with proper cryptographic verification
    let apple_client_id = std::env::var("APPLE_CLIENT_ID").map_err(|_| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Apple Sign-In not configured"})),
        )
    })?;

    // Use the secure validation function that fetches Apple's public keys
    let claims = validate_apple_id_token(&id_token, &apple_client_id, &stored_nonce).await?;

    // Get email from claims
    let email = claims.email.ok_or_else(|| {
        tracing::error!("Apple ID token missing email");
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Email not provided by Apple"})),
        )
    })?;

    // Parse user info if provided (only on first authentication)
    let user_info: Option<AppleUserInfo> = body.user.and_then(|u| serde_json::from_str(&u).ok());

    let name = user_info.and_then(|u| {
        u.name.and_then(|n| match (n.first_name, n.last_name) {
            (Some(first), Some(last)) => Some(format!("{first} {last}")),
            (Some(first), None) => Some(first),
            (None, Some(last)) => Some(last),
            (None, None) => None,
        })
    });

    // Create or get user
    let user = create_or_get_oauth_user(
        &state,
        OAuthProvider::Apple,
        &claims.sub,
        &email,
        name,
        None, // Apple doesn't provide profile pictures
        None,
        None,
    )
    .await?;

    // Create session in Redis
    if let Some(redis) = &state.services.redis {
        let session_id = generate_session_id();
        let _ = redis
            .create_session(&session_id, user.id, &user.email, None, None)
            .await;
    }

    // Create auth tokens
    let auth_response = create_oauth_auth_response(user, &state).await?;

    tracing::info!(
        target: "security_audit",
        event = "apple_oauth_success",
        user_id = %auth_response.user.id,
        "ICT 7 AUDIT: Apple Sign-In successful"
    );

    // FIX-C-1 (2026-04-29): tokens are now set as httpOnly cookies on the
    // redirect Response. See google_callback for full rationale.
    Ok(oauth_callback_response_with_cookies(
        &state,
        &auth_response,
        "apple",
    ))
}
