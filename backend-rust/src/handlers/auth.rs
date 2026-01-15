//! Authentication Handlers
//!
//! ICT 7 Principal Engineer Grade
//! JWT-based authentication with Argon2 password hashing
//! Security-hardened with token revocation

use axum::extract::Request;
use axum::{
    extract::State,
    http::{header, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    errors::AppError,
    models::User,
    responses::ApiResponse,
    services::{revoke_token, AuthService},
    AppState,
};

// =============================================================================
// Request/Response Types
// =============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, max = 255, message = "Name is required"))]
    pub name: String,
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// ICT 11+: Complete user response matching frontend User interface
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub roles: Vec<String>,
    pub is_verified: bool,
    pub email_verified_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl UserResponse {
    /// ICT 11+: Convert User model to UserResponse with all fields
    pub fn from_user(user: &User) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            avatar_url: user.avatar_url.clone(),
            role: user.role.clone(),
            roles: vec![user.role.clone()],
            is_verified: user.email_verified_at.is_some(),
            email_verified_at: user
                .email_verified_at
                .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            created_at: user.created_at.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            updated_at: user.updated_at.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        }
    }
}

// =============================================================================
// Handlers
// =============================================================================

/// POST /api/auth/register
#[tracing::instrument(skip(state, payload))]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<ApiResponse<AuthResponse>>), AppError> {
    // Validate request
    payload.validate()?;

    // Check password confirmation
    if payload.password != payload.password_confirmation {
        return Err(AppError::ValidationMessage(
            "Passwords do not match".to_string(),
        ));
    }

    let auth_service = AuthService::new(&state.db, &state.config.jwt);
    let (user, tokens) = auth_service
        .register(&payload.name, &payload.email, &payload.password)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(AuthResponse {
            user: UserResponse::from_user(&user),
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_in: tokens.expires_in,
        })),
    ))
}

/// POST /api/auth/login
#[tracing::instrument(skip(state, payload))]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    payload.validate()?;

    let auth_service =
        AuthService::with_developer_config(&state.db, &state.config.jwt, &state.config.developer);
    let (user, tokens) = auth_service
        .login(&payload.email, &payload.password)
        .await?;

    Ok(Json(ApiResponse::success(AuthResponse {
        user: UserResponse::from_user(&user),
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_in: tokens.expires_in,
    })))
}

/// POST /api/auth/logout
/// ICT 7 SECURITY: Revokes the access token to prevent further use
#[tracing::instrument(skip(state, request))]
pub async fn logout(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<serde_json::Value>, AppError> {
    // Extract the token from Authorization header
    if let Some(auth_header) = request.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                // ICT 7 SECURITY: Revoke the token
                // Access tokens expire in 15 minutes (900 seconds)
                revoke_token(&state.token_blacklist, token, 900).await;

                tracing::info!(
                    target: "security",
                    "Token revoked on logout"
                );
            }
        }
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Logged out successfully"
    })))
}

/// POST /api/auth/refresh
#[tracing::instrument(skip(state, payload))]
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let auth_service = AuthService::new(&state.db, &state.config.jwt);
    let (user, tokens) = auth_service.refresh_token(&payload.refresh_token).await?;

    Ok(Json(ApiResponse::success(AuthResponse {
        user: UserResponse::from_user(&user),
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_in: tokens.expires_in,
    })))
}

/// POST /api/auth/forgot-password
#[tracing::instrument(skip(state, payload))]
pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    payload.validate()?;

    let auth_service = AuthService::new(&state.db, &state.config.jwt);
    auth_service.send_password_reset(&payload.email).await?;

    // Always return success to prevent email enumeration
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "If an account exists with that email, a password reset link has been sent."
    })))
}

/// POST /api/auth/reset-password
#[tracing::instrument(skip(state, payload))]
pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    payload.validate()?;

    if payload.password != payload.password_confirmation {
        return Err(AppError::ValidationMessage(
            "Passwords do not match".to_string(),
        ));
    }

    let auth_service = AuthService::new(&state.db, &state.config.jwt);
    auth_service
        .reset_password(&payload.token, &payload.email, &payload.password)
        .await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Password has been reset successfully."
    })))
}

/// GET /api/auth/verify-email/:token
#[tracing::instrument(skip(state))]
pub async fn verify_email(
    State(state): State<AppState>,
    axum::extract::Path(token): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let auth_service = AuthService::new(&state.db, &state.config.jwt);
    auth_service.verify_email(&token).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Email verified successfully."
    })))
}
