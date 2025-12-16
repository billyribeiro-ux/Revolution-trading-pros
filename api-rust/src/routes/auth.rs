//! Authentication routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::user::{
    RegisterRequest, LoginRequest, MfaLoginRequest, 
    ForgotPasswordRequest, ResetPasswordRequest, AuthResponse, UserPublic
};
use crate::services::{JwtService, PasswordService};
use crate::utils;

/// POST /api/register - Register a new user
pub async fn register(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let body: RegisterRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    // Validate password strength
    PasswordService::validate_strength(&body.password)
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    // Check if email already exists
    let existing = ctx.data.db.query_one::<crate::models::User>(
        "SELECT * FROM users WHERE email = $1",
        vec![serde_json::json!(body.email.to_lowercase())]
    ).await?;

    if existing.is_some() {
        return Err(ApiError::Conflict("Email already registered".to_string()).into());
    }

    // Hash password
    let password_hash = PasswordService::hash(&body.password)?;

    // Create user
    let user_id = uuid::Uuid::new_v4();
    let now = utils::now();

    ctx.data.db.execute(
        r#"
        INSERT INTO users (id, email, name, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'user', $5, $5)
        "#,
        vec![
            serde_json::json!(user_id.to_string()),
            serde_json::json!(body.email.to_lowercase()),
            serde_json::json!(body.name),
            serde_json::json!(password_hash),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    // Generate tokens
    let jwt = JwtService::new(
        &ctx.data.config.jwt_secret,
        &ctx.data.config.jwt_issuer,
        &ctx.data.config.jwt_audience,
    );

    let (token, expires_at) = jwt.generate_access_token(
        &user_id.to_string(),
        &body.email,
        crate::models::user::UserRole::User,
    )?;

    let response = AuthResponse {
        user: UserPublic {
            id: user_id,
            email: body.email.to_lowercase(),
            name: body.name,
            role: crate::models::user::UserRole::User,
            avatar_url: None,
            email_verified: false,
            mfa_enabled: false,
            created_at: now,
        },
        token,
        refresh_token: None,
        expires_at: chrono::DateTime::from_timestamp(expires_at, 0).unwrap_or_else(|| utils::now()),
        mfa_required: false,
    };

    Response::from_json(&response).map(|r| r.with_status(201))
}

/// POST /api/login - Login with email and password
pub async fn login(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let body: LoginRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    // Find user by email
    let user = ctx.data.db.query_one::<crate::models::User>(
        "SELECT * FROM users WHERE email = $1",
        vec![serde_json::json!(body.email.to_lowercase())]
    ).await?
    .ok_or_else(|| ApiError::Unauthorized("Invalid credentials".to_string()))?;

    // Check if banned
    if user.banned_at.is_some() {
        return Err(ApiError::Forbidden("Account is banned".to_string()).into());
    }

    // Verify password
    let valid = PasswordService::verify(&body.password, &user.password_hash)?;
    if !valid {
        // Log failed attempt
        log_security_event(&ctx, &user.id, "login_failed", &req).await;
        return Err(ApiError::Unauthorized("Invalid credentials".to_string()).into());
    }

    // Check if MFA is enabled
    if user.mfa_enabled {
        // Return partial response requiring MFA
        return Response::from_json(&serde_json::json!({
            "mfa_required": true,
            "email": user.email
        }));
    }

    // Generate tokens
    let jwt = JwtService::new(
        &ctx.data.config.jwt_secret,
        &ctx.data.config.jwt_issuer,
        &ctx.data.config.jwt_audience,
    );

    let (token, expires_at) = jwt.generate_access_token(
        &user.id.to_string(),
        &user.email,
        user.role,
    )?;

    let (refresh_token, _) = jwt.generate_refresh_token(&user.id.to_string())?;

    // Update last login
    ctx.data.db.execute(
        "UPDATE users SET last_login_at = $1 WHERE id = $2",
        vec![
            serde_json::json!(utils::now_iso()),
            serde_json::json!(user.id.to_string()),
        ]
    ).await?;

    // Log successful login
    log_security_event(&ctx, &user.id, "login", &req).await;

    let response = AuthResponse {
        user: user.clone().into(),
        token,
        refresh_token: Some(refresh_token),
        expires_at: chrono::DateTime::from_timestamp(expires_at, 0).unwrap_or_else(|| utils::now()),
        mfa_required: false,
    };

    Response::from_json(&response)
}

/// POST /api/login/mfa - Complete login with MFA code
pub async fn login_mfa(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let body: MfaLoginRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    // Find user
    let user = ctx.data.db.query_one::<crate::models::User>(
        "SELECT * FROM users WHERE email = $1",
        vec![serde_json::json!(body.email.to_lowercase())]
    ).await?
    .ok_or_else(|| ApiError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify MFA code (TOTP)
    // In production, use a proper TOTP library
    let mfa_secret = user.mfa_secret.clone()
        .ok_or_else(|| ApiError::BadRequest("MFA not enabled".to_string()))?;

    // TODO: Implement proper TOTP verification
    if body.code != "000000" { // Placeholder
        return Err(ApiError::Unauthorized("Invalid MFA code".to_string()).into());
    }

    // Generate tokens
    let jwt = JwtService::new(
        &ctx.data.config.jwt_secret,
        &ctx.data.config.jwt_issuer,
        &ctx.data.config.jwt_audience,
    );

    let (token, expires_at) = jwt.generate_access_token(
        &user.id.to_string(),
        &user.email,
        user.role,
    )?;

    let (refresh_token, _) = jwt.generate_refresh_token(&user.id.to_string())?;

    let response = AuthResponse {
        user: user.into(),
        token,
        refresh_token: Some(refresh_token),
        expires_at: chrono::DateTime::from_timestamp(expires_at, 0).unwrap_or_else(|| utils::now()),
        mfa_required: false,
    };

    Response::from_json(&response)
}

/// POST /api/logout - Logout and invalidate session
pub async fn logout(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    // Get token from header
    let auth_header = req.headers().get("Authorization")?.unwrap_or_default();
    
    if let Some(token) = JwtService::extract_token(&auth_header) {
        // Invalidate token by adding to blacklist (with TTL)
        let _ = ctx.data.cache.set(
            &format!("blacklist:{}", token),
            &true,
            Some(3600 * 24), // 24 hours
        ).await;
    }

    Response::from_json(&serde_json::json!({
        "message": "Logged out successfully"
    }))
}

/// POST /api/auth/refresh - Refresh access token
pub async fn refresh_token(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    #[derive(serde::Deserialize)]
    struct RefreshRequest {
        refresh_token: String,
    }

    let body: RefreshRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    let jwt = JwtService::new(
        &ctx.data.config.jwt_secret,
        &ctx.data.config.jwt_issuer,
        &ctx.data.config.jwt_audience,
    );

    // Validate refresh token
    let user_id = jwt.validate_refresh_token(&body.refresh_token)?;

    // Get user
    let user = ctx.data.db.query_one::<crate::models::User>(
        "SELECT * FROM users WHERE id = $1",
        vec![serde_json::json!(user_id)]
    ).await?
    .ok_or_else(|| ApiError::Unauthorized("User not found".to_string()))?;

    // Generate new access token
    let (token, expires_at) = jwt.generate_access_token(
        &user.id.to_string(),
        &user.email,
        user.role,
    )?;

    Response::from_json(&serde_json::json!({
        "token": token,
        "expires_at": chrono::DateTime::from_timestamp(expires_at, 0)
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default()
    }))
}

/// POST /api/forgot-password - Request password reset
pub async fn forgot_password(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let body: ForgotPasswordRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    // Find user (don't reveal if email exists)
    let user = ctx.data.db.query_one::<crate::models::User>(
        "SELECT * FROM users WHERE email = $1",
        vec![serde_json::json!(body.email.to_lowercase())]
    ).await?;

    if let Some(user) = user {
        // Generate reset token
        let token = PasswordService::generate_token();
        let token_hash = PasswordService::hash_token(&token);
        let expires_at = utils::now() + chrono::Duration::hours(1);

        // Store reset token
        ctx.data.db.execute(
            r#"
            INSERT INTO password_resets (id, user_id, token_hash, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            vec![
                serde_json::json!(uuid::Uuid::new_v4().to_string()),
                serde_json::json!(user.id.to_string()),
                serde_json::json!(token_hash),
                serde_json::json!(expires_at.to_rfc3339()),
                serde_json::json!(utils::now_iso()),
            ]
        ).await?;

        // Send email (async, don't wait)
        // TODO: Queue email job
    }

    // Always return success to prevent email enumeration
    Response::from_json(&serde_json::json!({
        "message": "If an account exists with that email, a password reset link has been sent."
    }))
}

/// POST /api/reset-password - Reset password with token
pub async fn reset_password(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let body: ResetPasswordRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    if body.password != body.password_confirmation {
        return Err(ApiError::Validation("Passwords do not match".to_string()).into());
    }

    PasswordService::validate_strength(&body.password)?;

    // Find reset token
    let token_hash = PasswordService::hash_token(&body.token);
    
    let reset = ctx.data.db.query_one::<crate::models::user::PasswordReset>(
        r#"
        SELECT * FROM password_resets 
        WHERE token_hash = $1 AND used_at IS NULL AND expires_at > $2
        "#,
        vec![
            serde_json::json!(token_hash),
            serde_json::json!(utils::now_iso()),
        ]
    ).await?
    .ok_or_else(|| ApiError::BadRequest("Invalid or expired reset token".to_string()))?;

    // Hash new password
    let password_hash = PasswordService::hash(&body.password)?;

    // Update password
    ctx.data.db.execute(
        "UPDATE users SET password_hash = $1, updated_at = $2 WHERE id = $3",
        vec![
            serde_json::json!(password_hash),
            serde_json::json!(utils::now_iso()),
            serde_json::json!(reset.user_id.to_string()),
        ]
    ).await?;

    // Mark token as used
    ctx.data.db.execute(
        "UPDATE password_resets SET used_at = $1 WHERE id = $2",
        vec![
            serde_json::json!(utils::now_iso()),
            serde_json::json!(reset.id.to_string()),
        ]
    ).await?;

    Response::from_json(&serde_json::json!({
        "message": "Password reset successfully"
    }))
}

/// GET /api/auth/check - Check if token is valid
pub async fn check(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let auth_header = req.headers().get("Authorization")?.unwrap_or_default();
    
    let token = JwtService::extract_token(&auth_header)
        .ok_or_else(|| ApiError::Unauthorized("Missing token".to_string()))?;

    // Check blacklist
    let blacklisted: Option<bool> = ctx.data.cache.get(&format!("blacklist:{}", token)).await?;
    if blacklisted.unwrap_or(false) {
        return Err(ApiError::Unauthorized("Token revoked".to_string()).into());
    }

    let jwt = JwtService::new(
        &ctx.data.config.jwt_secret,
        &ctx.data.config.jwt_issuer,
        &ctx.data.config.jwt_audience,
    );

    let claims = jwt.validate_access_token(token)?;

    Response::from_json(&serde_json::json!({
        "valid": true,
        "user_id": claims.sub,
        "email": claims.email,
        "role": claims.role
    }))
}

/// Helper to log security events
async fn log_security_event(
    ctx: &RouteContext<AppState>,
    user_id: &uuid::Uuid,
    event_type: &str,
    req: &Request,
) {
    let ip = req.headers().get("CF-Connecting-IP").ok().flatten();
    let user_agent = req.headers().get("User-Agent").ok().flatten();

    let _ = ctx.data.db.execute(
        r#"
        INSERT INTO security_events (id, user_id, event_type, ip_address, user_agent, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        vec![
            serde_json::json!(uuid::Uuid::new_v4().to_string()),
            serde_json::json!(user_id.to_string()),
            serde_json::json!(event_type),
            serde_json::json!(ip),
            serde_json::json!(user_agent),
            serde_json::json!(utils::now_iso()),
        ]
    ).await;
}
