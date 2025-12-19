//! Authentication routes - December 2025 ICT11+ Principal Engineer Grade
//!
//! Implements complete authentication contract for frontend:
//! - POST /register - Create new user account (requires email verification)
//! - POST /login - Authenticate user (requires verified email)
//! - POST /refresh - Refresh access token
//! - POST /logout - Invalidate session
//! - GET /me - Get current user
//! - POST /forgot-password - Request password reset
//! - POST /reset-password - Reset password with token
//! - GET /verify-email - Verify email with token
//! - POST /resend-verification - Resend verification email

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::{
        AuthResponse, CreateUser, ForgotPasswordRequest, LoginUser, MessageResponse,
        RefreshTokenRequest, RefreshTokenResponse, ResetPasswordRequest, User, UserResponse,
    },
    utils::{
        create_jwt, create_refresh_token, generate_session_id, generate_verification_token,
        hash_password, hash_token, verify_jwt, verify_password, validate_password,
        hash_dummy_password,
    },
    AppState,
};

/// Query params for email verification
#[derive(Debug, Deserialize)]
struct VerifyEmailQuery {
    token: String,
}

/// Request to resend verification email
#[derive(Debug, Deserialize)]
struct ResendVerificationRequest {
    email: String,
}

/// Registration response (no tokens - must verify email first)
#[derive(Debug, serde::Serialize)]
struct RegisterResponse {
    message: String,
    email: String,
    requires_verification: bool,
}

/// Register a new user
/// POST /api/auth/register
async fn register(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Validate email format
    if !input.email.contains('@') || !input.email.contains('.') {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Validation failed",
                "errors": {
                    "email": ["Invalid email format"]
                }
            })),
        ));
    }

    // ICT L11+ Security: Validate password strength with hardened rules
    if let Err(password_error) = validate_password(&input.password) {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Validation failed",
                "errors": {
                    "password": [password_error]
                }
            })),
        ));
    }

    // Check if email exists
    let existing: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    if existing.is_some() {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Validation failed",
                "errors": {
                    "email": ["Email already registered"]
                }
            })),
        ));
    }

    // Hash password
    let password_hash = hash_password(&input.password).map_err(|e| {
        tracing::error!("Password hashing error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Password hashing failed"})),
        )
    })?;

    // Create user with email_verified_at = NULL (unverified)
    let user: User = sqlx::query_as(
        r#"
        INSERT INTO users (email, password, name, role, email_verified_at, created_at, updated_at)
        VALUES ($1, $2, $3, 'user', NULL, NOW(), NOW())
        RETURNING *
        "#,
    )
    .bind(&input.email)
    .bind(&password_hash)
    .bind(&input.name)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("User creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create user"})),
        )
    })?;

    // Generate verification token
    let (raw_token, hashed_token) = generate_verification_token();

    // Store verification token (expires in 24 hours)
    sqlx::query(
        r#"
        INSERT INTO email_verification_tokens (user_id, token, expires_at)
        VALUES ($1, $2, NOW() + INTERVAL '24 hours')
        "#,
    )
    .bind(user.id)
    .bind(&hashed_token)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create verification token: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create verification token"})),
        )
    })?;

    // Send verification email (if email service is configured)
    if let Some(ref email_service) = state.services.email {
        if let Err(e) = email_service
            .send_verification_email(&user.email, &user.name, &raw_token)
            .await
        {
            tracing::error!("Failed to send verification email: {}", e);
            // Don't fail registration if email fails - user can resend
        } else {
            tracing::info!("Verification email sent to: {}", user.email);
        }
    } else {
        tracing::warn!("Email service not configured - verification email not sent");
    }

    tracing::info!("User registered (pending verification): {}", user.email);

    // Return success without tokens - user must verify email first
    Ok(Json(RegisterResponse {
        message: "Registration successful! Please check your email to verify your account.".to_string(),
        email: user.email,
        requires_verification: true,
    }))
}

/// Verify email with token
/// GET /api/auth/verify-email?token=xxx
async fn verify_email(
    State(state): State<AppState>,
    Query(query): Query<VerifyEmailQuery>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    let hashed_token = hash_token(&query.token);

    // Find the verification token
    let token_record: Option<(i64, i64)> = sqlx::query_as(
        r#"
        SELECT id, user_id FROM email_verification_tokens 
        WHERE token = $1 AND expires_at > NOW()
        "#,
    )
    .bind(&hashed_token)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let (token_id, user_id) = token_record.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid or expired verification link"})),
        )
    })?;

    // Get user for welcome email
    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    // Update user's email_verified_at
    sqlx::query("UPDATE users SET email_verified_at = NOW(), updated_at = NOW() WHERE id = $1")
        .bind(user_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to verify user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to verify email"})),
            )
        })?;

    // Delete the used verification token
    sqlx::query("DELETE FROM email_verification_tokens WHERE id = $1")
        .bind(token_id)
        .execute(&state.db.pool)
        .await
        .ok(); // Ignore errors on cleanup

    // Send welcome email
    if let Some(ref email_service) = state.services.email {
        if let Err(e) = email_service.send_welcome_email(&user.email, &user.name).await {
            tracing::error!("Failed to send welcome email: {}", e);
            // Don't fail verification if welcome email fails
        }
    }

    tracing::info!("Email verified for user: {}", user.email);

    Ok(Json(MessageResponse {
        message: "Email verified successfully! You can now log in.".to_string(),
        success: Some(true),
    }))
}

/// Resend verification email
/// POST /api/auth/resend-verification
async fn resend_verification(
    State(state): State<AppState>,
    Json(input): Json<ResendVerificationRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Find user by email
    let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    // Always return success to prevent user enumeration
    let Some(user) = user else {
        return Ok(Json(MessageResponse {
            message: "If your email is registered, you will receive a verification link shortly."
                .to_string(),
            success: Some(true),
        }));
    };

    // Check if already verified
    if user.email_verified_at.is_some() {
        return Ok(Json(MessageResponse {
            message: "Your email is already verified. You can log in.".to_string(),
            success: Some(true),
        }));
    }

    // Delete any existing verification tokens for this user
    sqlx::query("DELETE FROM email_verification_tokens WHERE user_id = $1")
        .bind(user.id)
        .execute(&state.db.pool)
        .await
        .ok();

    // Generate new verification token
    let (raw_token, hashed_token) = generate_verification_token();

    // Store verification token
    sqlx::query(
        r#"
        INSERT INTO email_verification_tokens (user_id, token, expires_at)
        VALUES ($1, $2, NOW() + INTERVAL '24 hours')
        "#,
    )
    .bind(user.id)
    .bind(&hashed_token)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create verification token: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create verification token"})),
        )
    })?;

    // Send verification email
    if let Some(ref email_service) = state.services.email {
        if let Err(e) = email_service
            .send_verification_email(&user.email, &user.name, &raw_token)
            .await
        {
            tracing::error!("Failed to send verification email: {}", e);
        }
    }

    Ok(Json(MessageResponse {
        message: "If your email is registered, you will receive a verification link shortly."
            .to_string(),
        success: Some(true),
    }))
}

/// Login user
/// POST /api/auth/login
/// ICT L11+ Security: Timing attack prevention - always perform password hash
async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginUser>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Security audit logging
    tracing::info!(
        target: "security",
        event = "login_attempt",
        email = %input.email,
        "Login attempt initiated"
    );

    // Find user
    let user_result: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    // ICT L11+ Security: Timing attack prevention
    // Always perform password hashing even if user doesn't exist
    // This prevents timing-based user enumeration
    let user = match user_result {
        Some(u) => u,
        None => {
            // Hash dummy password to match timing of real verification
            hash_dummy_password();
            tracing::info!(
                target: "security",
                event = "login_failed",
                reason = "user_not_found",
                email = %input.email,
                "Login failed - user not found (timing protected)"
            );
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid credentials"})),
            ));
        }
    };

    // Verify password (this happens regardless of user existence due to above)
    let password_valid = verify_password(&input.password, &user.password_hash).map_err(|e| {
        tracing::error!("Password verification error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Authentication error"})),
        )
    })?;

    if !password_valid {
        tracing::info!(
            target: "security",
            event = "login_failed",
            reason = "invalid_password",
            user_id = %user.id,
            email = %user.email,
            "Login failed - invalid password"
        );
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ));
    }

    // Check if email is verified (after password check for security)
    if user.email_verified_at.is_none() {
        tracing::info!(
            target: "security",
            event = "login_failed",
            reason = "email_not_verified",
            user_id = %user.id,
            email = %user.email,
            "Login failed - email not verified"
        );
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "Please verify your email before logging in.",
                "code": "EMAIL_NOT_VERIFIED",
                "email": user.email
            })),
        ));
    }

    // Create access token
    let token = create_jwt(user.id, &state.config.jwt_secret, state.config.jwt_expires_in).map_err(
        |e| {
            tracing::error!("JWT creation error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Token creation failed"})),
            )
        },
    )?;

    // Create refresh token
    let refresh_token = create_refresh_token(user.id, &state.config.jwt_secret).map_err(|e| {
        tracing::error!("Refresh token creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Refresh token creation failed"})),
        )
    })?;

    // Generate session ID
    let session_id = generate_session_id();

    // Security audit: successful login
    tracing::info!(
        target: "security",
        event = "login_success",
        user_id = %user.id,
        email = %user.email,
        "Login successful"
    );

    Ok(Json(AuthResponse {
        token,
        refresh_token,
        session_id,
        user: user.into(),
        expires_in: state.config.jwt_expires_in * 3600, // Convert hours to seconds
    }))
}

/// Refresh access token
/// POST /api/auth/refresh
async fn refresh(
    State(state): State<AppState>,
    Json(input): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Verify the refresh token
    let claims = verify_jwt(&input.refresh_token, &state.config.jwt_secret).map_err(|e| {
        tracing::warn!("Invalid refresh token: {}", e);
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid or expired refresh token"})),
        )
    })?;

    // Verify user still exists
    let user_exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE id = $1")
        .bind(claims.sub)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    if user_exists.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "User not found"})),
        ));
    }

    // Create new access token
    let token =
        create_jwt(claims.sub, &state.config.jwt_secret, state.config.jwt_expires_in).map_err(
            |e| {
                tracing::error!("JWT creation error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Token creation failed"})),
                )
            },
        )?;

    // Create new refresh token (token rotation for security)
    let new_refresh_token = create_refresh_token(claims.sub, &state.config.jwt_secret).map_err(
        |e| {
            tracing::error!("Refresh token creation error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Refresh token creation failed"})),
            )
        },
    )?;

    tracing::debug!("Token refreshed for user: {}", claims.sub);

    Ok(Json(RefreshTokenResponse {
        token,
        refresh_token: new_refresh_token,
        expires_in: state.config.jwt_expires_in * 3600,
    }))
}

/// Get current user (requires auth)
/// GET /api/auth/me
async fn me(user: User) -> Json<UserResponse> {
    Json(user.into())
}

/// Logout user (invalidates token on client side)
/// POST /api/auth/logout
async fn logout() -> Json<MessageResponse> {
    // JWT tokens are stateless - logout is handled client-side
    // In a production app with Redis, you might want to add the token to a blacklist
    Json(MessageResponse {
        message: "Logged out successfully".to_string(),
        success: Some(true),
    })
}

/// Request password reset
/// POST /api/auth/forgot-password
async fn forgot_password(
    State(state): State<AppState>,
    Json(input): Json<ForgotPasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Find user (but don't reveal if they exist)
    let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    // If user exists, create reset token and send email
    if let Some(user) = user {
        // Delete any existing reset tokens for this user
        sqlx::query("DELETE FROM password_resets WHERE email = $1")
            .bind(&user.email)
            .execute(&state.db.pool)
            .await
            .ok();

        // Generate reset token
        let (raw_token, hashed_token) = generate_verification_token();

        // Store reset token (expires in 1 hour)
        sqlx::query(
            r#"
            INSERT INTO password_resets (id, email, token, expires_at, created_at)
            VALUES (gen_random_uuid(), $1, $2, NOW() + INTERVAL '1 hour', NOW())
            "#,
        )
        .bind(&user.email)
        .bind(&hashed_token)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create reset token: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create reset token"})),
            )
        })?;

        // Send password reset email
        if let Some(ref email_service) = state.services.email {
            if let Err(e) = email_service
                .send_password_reset(&user.email, &user.name, &raw_token)
                .await
            {
                tracing::error!("Failed to send password reset email: {}", e);
            } else {
                tracing::info!("Password reset email sent to: {}", user.email);
            }
        }
    }

    // Always return success to prevent user enumeration
    Ok(Json(MessageResponse {
        message: "If your email is registered, you will receive a password reset link shortly."
            .to_string(),
        success: Some(true),
    }))
}

/// Reset password with token
/// POST /api/auth/reset-password
async fn reset_password(
    State(state): State<AppState>,
    Json(input): Json<ResetPasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Validate passwords match
    if input.password != input.password_confirmation {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Validation failed",
                "errors": {
                    "password_confirmation": ["Passwords do not match"]
                }
            })),
        ));
    }

    // ICT L11+ Security: Validate password strength with hardened rules
    if let Err(password_error) = validate_password(&input.password) {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Validation failed",
                "errors": {
                    "password": [password_error]
                }
            })),
        ));
    }

    // Hash the token and find it in the database
    let hashed_token = hash_token(&input.token);

    let reset_record: Option<(uuid::Uuid, String)> = sqlx::query_as(
        r#"
        SELECT id, email FROM password_resets 
        WHERE token = $1 AND email = $2 AND expires_at > NOW()
        "#,
    )
    .bind(&hashed_token)
    .bind(&input.email)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let (reset_id, _email) = reset_record.ok_or_else(|| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Invalid or expired reset token",
                "errors": {
                    "token": ["Invalid reset token"]
                }
            })),
        )
    })?;

    // Hash the new password
    let password_hash = hash_password(&input.password).map_err(|e| {
        tracing::error!("Password hashing error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Password hashing failed"})),
        )
    })?;

    // Update user's password
    let result = sqlx::query("UPDATE users SET password = $1, updated_at = NOW() WHERE email = $2")
        .bind(&password_hash)
        .bind(&input.email)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Failed to reset password",
                "errors": {
                    "email": ["User not found"]
                }
            })),
        ));
    }

    // Delete the used reset token
    sqlx::query("DELETE FROM password_resets WHERE id = $1")
        .bind(reset_id)
        .execute(&state.db.pool)
        .await
        .ok();

    tracing::info!("Password reset completed for: {}", input.email);

    Ok(Json(MessageResponse {
        message: "Password has been reset successfully. You can now login with your new password."
            .to_string(),
        success: Some(true),
    }))
}

/// Build the auth router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/me", get(me))
        .route("/logout", post(logout))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
        .route("/verify-email", get(verify_email))
        .route("/resend-verification", post(resend_verification))
}
