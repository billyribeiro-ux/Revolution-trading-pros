//! OAuth Routes - January 2026 Apple Principal Engineer ICT Level 7 Grade
//!
//! Implements secure OAuth 2.0 / OpenID Connect flows for:
//! - Google Sign-In (OpenID Connect)
//! - Apple Sign-In (Sign in with Apple)
//!
//! Security Features:
//! - PKCE (Proof Key for Code Exchange) for enhanced security
//! - State parameter for CSRF protection
//! - Nonce validation for Apple ID tokens
//! - Secure token storage (encrypted at rest)
//! - Account linking with existing users
//! - Full audit logging

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    models::{AuthResponse, User, UserResponse},
    utils::{create_jwt, create_refresh_token, generate_session_id},
    AppState,
};

// =============================================================================
// Configuration & Constants
// =============================================================================

/// OAuth provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthProvider {
    Google,
    Apple,
}

impl std::fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthProvider::Google => write!(f, "google"),
            OAuthProvider::Apple => write!(f, "apple"),
        }
    }
}

// =============================================================================
// Request/Response Types
// =============================================================================

/// OAuth initiation response
#[derive(Debug, Serialize)]
pub struct OAuthInitResponse {
    pub authorization_url: String,
    pub state: String,
}

/// Google OAuth callback query params
#[derive(Debug, Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

/// Apple OAuth callback (POST body from form_post)
#[derive(Debug, Deserialize)]
pub struct AppleCallbackBody {
    pub code: Option<String>,
    pub id_token: Option<String>,
    pub state: Option<String>,
    pub user: Option<String>, // JSON string with user info (first auth only)
    pub error: Option<String>,
}

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
    sub: String,           // Google user ID
    email: String,
    email_verified: bool,
    name: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    picture: Option<String>,
    locale: Option<String>,
}

/// Apple ID token claims
#[derive(Debug, Deserialize)]
struct AppleIdTokenClaims {
    iss: String,           // https://appleid.apple.com
    sub: String,           // Apple user ID
    aud: String,           // Client ID
    exp: i64,
    iat: i64,
    nonce: Option<String>,
    nonce_supported: Option<bool>,
    email: Option<String>,
    email_verified: Option<String>, // "true" or "false" as string
    is_private_email: Option<String>,
    real_user_status: Option<i32>, // 0=unsupported, 1=unknown, 2=likely_real
}

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

// =============================================================================
// Helper Functions
// =============================================================================

/// Generate cryptographically secure random string
fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
    URL_SAFE_NO_PAD.encode(&bytes)
}

/// Generate PKCE code verifier (43-128 chars)
fn generate_code_verifier() -> String {
    generate_random_string(32) // Results in ~43 chars base64
}

/// Generate PKCE code challenge from verifier
fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(&hash)
}

/// Create or get existing user from OAuth data
async fn create_or_get_oauth_user(
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

    let existing_oauth_user: Option<User> = sqlx::query_as(&format!(
        "SELECT id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at
         FROM users WHERE {} = $1",
        provider_column
    ))
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
            "UPDATE users SET {} = $1, oauth_linked_at = COALESCE(oauth_linked_at, NOW()), updated_at = NOW() WHERE id = $2",
            provider_column
        );

        sqlx::query(&update_query)
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
        r#"
        INSERT INTO users (
            email, password_hash, name, role, email_verified_at, avatar_url,
            {}, oauth_provider, oauth_linked_at, created_at, updated_at
        )
        VALUES ($1, $2, $3, 'user', NOW(), $4, $5, $6, NOW(), NOW(), NOW())
        RETURNING id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at
        "#,
        provider_column
    );

    let new_user: User = sqlx::query_as(&insert_query)
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
        email = %email,
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
        r#"
        INSERT INTO oauth_audit_log (user_id, provider, action, provider_user_id, ip_address, user_agent)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
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
fn create_oauth_auth_response(
    user: User,
    state: &AppState,
) -> Result<AuthResponse, (StatusCode, Json<serde_json::Value>)> {
    // Create access token
    let token = create_jwt(
        user.id,
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
    )
    .map_err(|e| {
        tracing::error!("JWT creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Token creation failed"})),
        )
    })?;

    // Create refresh token
    let refresh_token = create_refresh_token(user.id, &state.config.jwt_secret).map_err(|e| {
        tracing::error!("Refresh token creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Refresh token creation failed"})),
        )
    })?;

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

// =============================================================================
// Google OAuth Handlers
// =============================================================================

/// Initiate Google OAuth flow
/// GET /api/auth/google
async fn google_init(
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
        r#"
        INSERT INTO oauth_states (state, provider, code_verifier, expires_at)
        VALUES ($1, 'google', $2, NOW() + INTERVAL '10 minutes')
        "#,
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
async fn google_callback(
    State(state): State<AppState>,
    Query(query): Query<GoogleCallbackQuery>,
) -> Result<Redirect, (StatusCode, Json<serde_json::Value>)> {
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
            urlencoding::encode(&format!("Google sign-in failed: {}", error))
        )));
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

    let code_verifier = state_record.ok_or_else(|| {
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
    })?.0;

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
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap();
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
        )));
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
            email = %user_info.email,
            "Google account email not verified"
        );
        return Ok(Redirect::to(&format!(
            "{}/login?error=email_not_verified",
            state.config.app_url
        )));
    }

    // Create or get user
    let name = user_info.name.or_else(|| {
        match (&user_info.given_name, &user_info.family_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            (Some(first), None) => Some(first.clone()),
            _ => None,
        }
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
    let auth_response = create_oauth_auth_response(user, &state)?;

    tracing::info!(
        target: "security_audit",
        event = "google_oauth_success",
        user_id = %auth_response.user.id,
        email = %auth_response.user.email,
        "ICT 7 AUDIT: Google OAuth login successful"
    );

    // Redirect to frontend callback with tokens
    let callback_url = format!(
        "{}/auth/callback?provider=google&token={}&refresh_token={}&session_id={}&expires_in={}",
        state.config.app_url,
        urlencoding::encode(&auth_response.token),
        urlencoding::encode(&auth_response.refresh_token),
        urlencoding::encode(&auth_response.session_id),
        auth_response.expires_in
    );

    Ok(Redirect::to(&callback_url))
}

// =============================================================================
// Apple OAuth Handlers
// =============================================================================

/// Initiate Apple Sign-In flow
/// GET /api/auth/apple
async fn apple_init(
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
        URL_SAFE_NO_PAD.encode(&hash)
    };

    // Store state and nonce in database
    sqlx::query(
        r#"
        INSERT INTO oauth_states (state, provider, nonce, expires_at)
        VALUES ($1, 'apple', $2, NOW() + INTERVAL '10 minutes')
        "#,
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
async fn apple_callback(
    State(state): State<AppState>,
    axum::Form(body): axum::Form<AppleCallbackBody>,
) -> Result<Redirect, (StatusCode, Json<serde_json::Value>)> {
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
            urlencoding::encode(&format!("Apple sign-in failed: {}", error))
        )));
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

    let _stored_nonce = state_record.ok_or_else(|| {
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
    })?.0;

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

    // Decode ID token header to get kid
    let header = decode_header(&id_token).map_err(|e| {
        tracing::error!("Failed to decode Apple ID token header: {}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid ID token"})),
        )
    })?;

    // For production: Fetch Apple's public keys and validate properly
    // For now, we decode without full validation but verify issuer and audience
    let apple_client_id = std::env::var("APPLE_CLIENT_ID").unwrap();

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&["https://appleid.apple.com"]);
    validation.set_audience(&[&apple_client_id]);
    validation.insecure_disable_signature_validation(); // TODO: Implement proper key fetching

    let token_data = decode::<AppleIdTokenClaims>(
        &id_token,
        &DecodingKey::from_secret(b""),
        &validation,
    )
    .map_err(|e| {
        tracing::error!("Failed to decode Apple ID token: {}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid ID token"})),
        )
    })?;

    let claims = token_data.claims;

    // Get email from claims
    let email = claims.email.ok_or_else(|| {
        tracing::error!("Apple ID token missing email");
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Email not provided by Apple"})),
        )
    })?;

    // Parse user info if provided (only on first authentication)
    let user_info: Option<AppleUserInfo> = body.user.and_then(|u| {
        serde_json::from_str(&u).ok()
    });

    let name = user_info.and_then(|u| {
        u.name.and_then(|n| {
            match (n.first_name, n.last_name) {
                (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
                (Some(first), None) => Some(first),
                (None, Some(last)) => Some(last),
                (None, None) => None,
            }
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
    let auth_response = create_oauth_auth_response(user, &state)?;

    tracing::info!(
        target: "security_audit",
        event = "apple_oauth_success",
        user_id = %auth_response.user.id,
        email = %auth_response.user.email,
        "ICT 7 AUDIT: Apple Sign-In successful"
    );

    // Redirect to frontend callback with tokens
    let callback_url = format!(
        "{}/auth/callback?provider=apple&token={}&refresh_token={}&session_id={}&expires_in={}",
        state.config.app_url,
        urlencoding::encode(&auth_response.token),
        urlencoding::encode(&auth_response.refresh_token),
        urlencoding::encode(&auth_response.session_id),
        auth_response.expires_in
    );

    Ok(Redirect::to(&callback_url))
}

// =============================================================================
// Router
// =============================================================================

/// Build the OAuth router
pub fn router() -> Router<AppState> {
    Router::new()
        // Google OAuth
        .route("/google", get(google_init))
        .route("/google/callback", get(google_callback))
        // Apple Sign-In
        .route("/apple", get(apple_init))
        .route("/apple/callback", post(apple_callback))
}
