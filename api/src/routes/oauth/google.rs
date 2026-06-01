//! Google Sign-In (OpenID Connect) — init + callback handlers.
//!
//! Security contract:
//! - `state` parameter is generated server-side, persisted to `oauth_states`,
//!   and rejected on mismatch / expiry / re-use (`used_at IS NULL` filter).
//! - PKCE: a fresh `code_verifier` is stored per `state` row and replayed
//!   on the token exchange call to Google.
//! - Email must be verified by Google (`email_verified == true`) before we
//!   accept the identity.
//! - Tokens are returned via httpOnly cookies on the redirect Response
//!   (FIX-C-1, 2026-04-29) — never in the URL.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use serde::Deserialize;

use super::crypto::{generate_code_challenge, generate_code_verifier, generate_random_string};
use super::session::oauth_callback_response_with_cookies;
use super::types::{GoogleCallbackQuery, OAuthProvider};
use super::user::{create_oauth_auth_response, create_or_get_oauth_user};
use crate::{utils::generate_session_id, AppState};

/// Google token response
#[derive(Debug, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
    id_token: String,
    expires_in: i64,
    token_type: String,
    scope: Option<String>,
    refresh_token: Option<String>,
}

/// Google user info
#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    sub: String, // Google user ID
    email: String,
    email_verified: bool,
    name: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    picture: Option<String>,
    locale: Option<String>,
}

/// Initiate Google OAuth flow
/// GET /api/auth/google
pub(super) async fn google_init(
    State(state): State<AppState>,
) -> Result<Redirect, (StatusCode, Json<serde_json::Value>)> {
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID").map_err(|_| {
        tracing::error!("GOOGLE_CLIENT_ID not configured");
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Google Sign-In not configured"})),
        )
    })?;

    // Generate state for CSRF protection
    let oauth_state = generate_random_string(32);

    // Generate PKCE code verifier and challenge
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);

    // Store state in database
    sqlx::query(
        r"
        INSERT INTO oauth_states (state, provider, code_verifier, expires_at)
        VALUES ($1, 'google', $2, NOW() + INTERVAL '10 minutes')
        ",
    )
    .bind(&oauth_state)
    .bind(&code_verifier)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to store OAuth state: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "OAuth initialization failed"})),
        )
    })?;

    // Build Google authorization URL
    let redirect_uri = format!("{}/api/auth/google/callback", state.config.app_url);
    let authorization_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?\
        client_id={}&\
        redirect_uri={}&\
        response_type=code&\
        scope=openid%20email%20profile&\
        state={}&\
        code_challenge={}&\
        code_challenge_method=S256&\
        access_type=offline&\
        prompt=consent",
        urlencoding::encode(&google_client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&oauth_state),
        urlencoding::encode(&code_challenge),
    );

    tracing::info!(
        target: "security_audit",
        event = "google_oauth_initiated",
        state = %oauth_state,
        "ICT 7 AUDIT: Google OAuth flow initiated"
    );

    Ok(Redirect::to(&authorization_url))
}

/// Handle Google OAuth callback
/// GET /api/auth/google/callback
pub(super) async fn google_callback(
    State(state): State<AppState>,
    Query(query): Query<GoogleCallbackQuery>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // FIX-C-1 (2026-04-29): return type changed from Redirect to Response so
    // we can attach Set-Cookie headers on the success path. Error paths
    // still issue plain redirects via .into_response().
    // Check for errors from Google
    if let Some(error) = query.error {
        let description = query.error_description.unwrap_or_default();
        tracing::warn!(
            target: "security_audit",
            event = "google_oauth_error",
            error = %error,
            description = %description,
            "Google OAuth error"
        );
        return Ok(Redirect::to(&format!(
            "{}/login?error={}",
            state.config.app_url,
            urlencoding::encode(&format!("Google sign-in failed: {error}"))
        ))
        .into_response());
    }

    // Validate state parameter
    let oauth_state = query.state.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing state parameter"})),
        )
    })?;

    // Get and validate stored state
    let state_record: Option<(String,)> = sqlx::query_as(
        "SELECT code_verifier FROM oauth_states WHERE state = $1 AND provider = 'google' AND expires_at > NOW() AND used_at IS NULL"
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

    let code_verifier = state_record
        .ok_or_else(|| {
            tracing::warn!(
                target: "security_audit",
                event = "google_oauth_invalid_state",
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

    // Get authorization code
    let code = query.code.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing authorization code"})),
        )
    })?;

    // Exchange code for tokens
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "OAuth not configured"})),
        )
    })?;
    let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET").map_err(|_| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "Google OAuth not configured"})),
        )
    })?;
    let redirect_uri = format!("{}/api/auth/google/callback", state.config.app_url);

    let token_response = reqwest::Client::new()
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code.as_str()),
            ("client_id", &google_client_id),
            ("client_secret", &google_client_secret),
            ("redirect_uri", &redirect_uri),
            ("grant_type", "authorization_code"),
            ("code_verifier", &code_verifier),
        ])
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Google token exchange failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": "Failed to authenticate with Google"})),
            )
        })?;

    if !token_response.status().is_success() {
        let error_text = token_response.text().await.unwrap_or_default();
        tracing::error!("Google token error: {}", error_text);
        return Ok(Redirect::to(&format!(
            "{}/login?error=google_token_failed",
            state.config.app_url
        ))
        .into_response());
    }

    let tokens: GoogleTokenResponse = token_response.json().await.map_err(|e| {
        tracing::error!("Failed to parse Google token response: {}", e);
        (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": "Invalid response from Google"})),
        )
    })?;

    // Get user info from Google
    let user_info_response = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(&tokens.access_token)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to get Google user info: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": "Failed to get user info from Google"})),
            )
        })?;

    let user_info: GoogleUserInfo = user_info_response.json().await.map_err(|e| {
        tracing::error!("Failed to parse Google user info: {}", e);
        (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": "Invalid user info from Google"})),
        )
    })?;

    // ICT 7: Require verified email
    if !user_info.email_verified {
        tracing::warn!(
            target: "security_audit",
            event = "google_oauth_unverified_email",
            "Google account email not verified"
        );
        return Ok(Redirect::to(&format!(
            "{}/login?error=email_not_verified",
            state.config.app_url
        ))
        .into_response());
    }

    // Create or get user
    let name = user_info
        .name
        .or_else(|| match (&user_info.given_name, &user_info.family_name) {
            (Some(first), Some(last)) => Some(format!("{first} {last}")),
            (Some(first), None) => Some(first.clone()),
            _ => None,
        });

    let user = create_or_get_oauth_user(
        &state,
        OAuthProvider::Google,
        &user_info.sub,
        &user_info.email,
        name,
        user_info.picture,
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
        event = "google_oauth_success",
        user_id = %auth_response.user.id,
        "Google OAuth login successful"
    );

    // FIX-C-1 (2026-04-29): tokens are now set as httpOnly cookies on the
    // redirect Response, NOT placed in the URL query string. The frontend
    // /auth/callback page no longer reads tokens from `?token=...`; it
    // calls GET /api/auth/me to confirm the session. See
    // oauth_callback_response_with_cookies() above for cookie attributes.
    Ok(oauth_callback_response_with_cookies(
        &state,
        &auth_response,
        "google",
    ))
}
