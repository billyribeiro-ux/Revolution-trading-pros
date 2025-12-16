//! User routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::user::{ChangePasswordRequest, UserPublic, SessionInfo, UserMembership};
use crate::services::{JwtService, PasswordService};
use crate::utils;

/// GET /api/me - Get current user profile
pub async fn me(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;
    Response::from_json(&UserPublic::from(user))
}

/// GET /api/me/memberships - Get user's active memberships
pub async fn memberships(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;

    let memberships: Vec<UserMembership> = ctx.data.db.query(
        r#"
        SELECT 
            pu.id, pu.user_id, pu.product_id, p.name as product_name, 
            p.product_type, pu.status, pu.starts_at, pu.expires_at, pu.created_at
        FROM product_user pu
        JOIN products p ON p.id = pu.product_id
        WHERE pu.user_id = $1 AND pu.status = 'active'
        ORDER BY pu.created_at DESC
        "#,
        vec![serde_json::json!(user.id.to_string())]
    ).await?;

    Response::from_json(&memberships)
}

/// GET /api/me/products - Get user's purchased products
pub async fn products(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;

    let products: Vec<crate::models::Product> = ctx.data.db.query(
        r#"
        SELECT p.* FROM products p
        JOIN product_user pu ON pu.product_id = p.id
        WHERE pu.user_id = $1 AND pu.status = 'active'
        ORDER BY pu.created_at DESC
        "#,
        vec![serde_json::json!(user.id.to_string())]
    ).await?;

    Response::from_json(&products)
}

/// GET /api/me/sessions - Get user's active sessions
pub async fn sessions(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;
    
    // Get current session token for comparison
    let auth_header = req.headers().get("Authorization")?.unwrap_or_default();
    let current_token = JwtService::extract_token(&auth_header).unwrap_or("");
    let current_token_hash = PasswordService::hash_token(current_token);

    let sessions: Vec<crate::models::user::Session> = ctx.data.db.query(
        r#"
        SELECT * FROM sessions 
        WHERE user_id = $1 AND expires_at > $2
        ORDER BY last_active_at DESC
        "#,
        vec![
            serde_json::json!(user.id.to_string()),
            serde_json::json!(utils::now_iso()),
        ]
    ).await?;

    let session_infos: Vec<SessionInfo> = sessions.into_iter().map(|s| {
        SessionInfo {
            id: s.id,
            device_name: s.device_name,
            ip_address: s.ip_address,
            last_active_at: s.last_active_at,
            is_current: s.token_hash == current_token_hash,
            created_at: s.created_at,
        }
    }).collect();

    Response::from_json(&session_infos)
}

/// PUT /api/me/password - Change password
pub async fn change_password(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;
    
    let body: ChangePasswordRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    if body.new_password != body.new_password_confirmation {
        return Err(ApiError::Validation("Passwords do not match".to_string()).into());
    }

    // Verify current password
    let valid = PasswordService::verify(&body.current_password, &user.password_hash)?;
    if !valid {
        return Err(ApiError::Unauthorized("Current password is incorrect".to_string()).into());
    }

    // Validate new password
    PasswordService::validate_strength(&body.new_password)?;

    // Hash and update
    let new_hash = PasswordService::hash(&body.new_password)?;
    
    ctx.data.db.execute(
        "UPDATE users SET password_hash = $1, updated_at = $2 WHERE id = $3",
        vec![
            serde_json::json!(new_hash),
            serde_json::json!(utils::now_iso()),
            serde_json::json!(user.id.to_string()),
        ]
    ).await?;

    Response::from_json(&serde_json::json!({
        "message": "Password changed successfully"
    }))
}

/// POST /api/me/mfa/enable - Enable MFA
pub async fn enable_mfa(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;

    if user.mfa_enabled {
        return Err(ApiError::BadRequest("MFA is already enabled".to_string()).into());
    }

    // Generate TOTP secret
    let secret = PasswordService::generate_token();
    
    // Store secret (not enabled yet until verified)
    ctx.data.db.execute(
        "UPDATE users SET mfa_secret = $1, updated_at = $2 WHERE id = $3",
        vec![
            serde_json::json!(secret),
            serde_json::json!(utils::now_iso()),
            serde_json::json!(user.id.to_string()),
        ]
    ).await?;

    // Return secret for QR code generation
    Response::from_json(&serde_json::json!({
        "secret": secret,
        "qr_url": format!(
            "otpauth://totp/RevolutionTradingPros:{}?secret={}&issuer=RevolutionTradingPros",
            user.email, secret
        )
    }))
}

/// POST /api/me/mfa/disable - Disable MFA
pub async fn disable_mfa(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;

    #[derive(serde::Deserialize)]
    struct DisableMfaRequest {
        password: String,
    }

    let body: DisableMfaRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    // Verify password
    let valid = PasswordService::verify(&body.password, &user.password_hash)?;
    if !valid {
        return Err(ApiError::Unauthorized("Invalid password".to_string()).into());
    }

    // Disable MFA
    ctx.data.db.execute(
        "UPDATE users SET mfa_enabled = false, mfa_secret = NULL, updated_at = $1 WHERE id = $2",
        vec![
            serde_json::json!(utils::now_iso()),
            serde_json::json!(user.id.to_string()),
        ]
    ).await?;

    Response::from_json(&serde_json::json!({
        "message": "MFA disabled successfully"
    }))
}

/// DELETE /api/me/sessions/:session_id - Revoke a session
pub async fn revoke_session(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;
    let session_id = ctx.param("session_id")
        .ok_or_else(|| ApiError::BadRequest("Missing session_id".to_string()))?;

    // Delete session (only if belongs to user)
    let deleted = ctx.data.db.execute(
        "DELETE FROM sessions WHERE id = $1 AND user_id = $2",
        vec![
            serde_json::json!(session_id),
            serde_json::json!(user.id.to_string()),
        ]
    ).await?;

    if deleted == 0 {
        return Err(ApiError::NotFound("Session not found".to_string()).into());
    }

    Response::from_json(&serde_json::json!({
        "message": "Session revoked successfully"
    }))
}

/// Helper to get authenticated user from request
async fn get_authenticated_user(
    req: &Request,
    ctx: &RouteContext<AppState>,
) -> Result<crate::models::User, ApiError> {
    let auth_header = req.headers().get("Authorization")
        .map_err(|_| ApiError::Unauthorized("Missing authorization header".to_string()))?
        .ok_or_else(|| ApiError::Unauthorized("Missing authorization header".to_string()))?;

    let token = JwtService::extract_token(&auth_header)
        .ok_or_else(|| ApiError::Unauthorized("Invalid authorization header".to_string()))?;

    // Check blacklist
    let blacklisted: Option<bool> = ctx.data.cache.get(&format!("blacklist:{}", token)).await
        .map_err(|_| ApiError::Internal("Cache error".to_string()))?;
    
    if blacklisted.unwrap_or(false) {
        return Err(ApiError::Unauthorized("Token revoked".to_string()));
    }

    let jwt = JwtService::new(
        &ctx.data.config.jwt_secret,
        &ctx.data.config.jwt_issuer,
        &ctx.data.config.jwt_audience,
    );

    let claims = jwt.validate_access_token(token)?;

    // Get user from database
    let user = ctx.data.db.query_one::<crate::models::User>(
        "SELECT * FROM users WHERE id = $1",
        vec![serde_json::json!(claims.sub)]
    ).await
    .map_err(|e| ApiError::Database(e.to_string()))?
    .ok_or_else(|| ApiError::Unauthorized("User not found".to_string()))?;

    // Check if banned
    if user.banned_at.is_some() {
        return Err(ApiError::Forbidden("Account is banned".to_string()));
    }

    Ok(user)
}
