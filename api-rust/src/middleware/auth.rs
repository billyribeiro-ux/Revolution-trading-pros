//! Authentication middleware

use worker::{Request, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::user::User;
use crate::services::JwtService;

/// Require authentication - returns the authenticated user
pub async fn require_auth(
    req: &Request,
    ctx: &RouteContext<AppState>,
) -> Result<User, ApiError> {
    let auth_header = req.headers().get("Authorization")
        .map_err(|_| ApiError::Unauthorized("Missing authorization header".to_string()))?
        .ok_or_else(|| ApiError::Unauthorized("Missing authorization header".to_string()))?;

    let token = JwtService::extract_token(&auth_header)
        .ok_or_else(|| ApiError::Unauthorized("Invalid authorization header format".to_string()))?;

    // Check token blacklist
    let blacklisted: Option<bool> = ctx.data.cache.get(&format!("blacklist:{}", token)).await
        .map_err(|_| ApiError::Internal("Cache error".to_string()))?;
    
    if blacklisted.unwrap_or(false) {
        return Err(ApiError::Unauthorized("Token has been revoked".to_string()));
    }

    // Validate token
    let jwt = JwtService::new(
        &ctx.data.config.jwt_secret,
        &ctx.data.config.jwt_issuer,
        &ctx.data.config.jwt_audience,
    );

    let claims = jwt.validate_access_token(token)?;

    // Get user from database
    let user = ctx.data.db.query_one::<User>(
        "SELECT * FROM users WHERE id = $1",
        vec![serde_json::json!(claims.sub)]
    ).await
    .map_err(|e| ApiError::Database(e.to_string()))?
    .ok_or_else(|| ApiError::Unauthorized("User not found".to_string()))?;

    // Check if user is banned
    if user.banned_at.is_some() {
        return Err(ApiError::Forbidden(format!(
            "Account is banned{}",
            user.ban_reason.as_ref().map(|r| format!(": {}", r)).unwrap_or_default()
        )));
    }

    Ok(user)
}

/// Require admin role - returns the authenticated admin user
pub async fn require_admin(
    req: &Request,
    ctx: &RouteContext<AppState>,
) -> Result<User, ApiError> {
    let user = require_auth(req, ctx).await?;

    if !user.role.is_admin() {
        return Err(ApiError::Forbidden("Admin access required".to_string()));
    }

    Ok(user)
}

/// Require super admin role - returns the authenticated super admin user
pub async fn require_super_admin(
    req: &Request,
    ctx: &RouteContext<AppState>,
) -> Result<User, ApiError> {
    let user = require_auth(req, ctx).await?;

    if !user.role.is_super_admin() {
        return Err(ApiError::Forbidden("Super admin access required".to_string()));
    }

    Ok(user)
}

/// Optional authentication - returns Some(user) if authenticated, None otherwise
pub async fn optional_auth(
    req: &Request,
    ctx: &RouteContext<AppState>,
) -> Option<User> {
    require_auth(req, ctx).await.ok()
}

/// Check if the current user owns a resource or is admin
pub async fn require_owner_or_admin(
    req: &Request,
    ctx: &RouteContext<AppState>,
    resource_user_id: &uuid::Uuid,
) -> Result<User, ApiError> {
    let user = require_auth(req, ctx).await?;

    if user.id != *resource_user_id && !user.role.is_admin() {
        return Err(ApiError::Forbidden("You don't have permission to access this resource".to_string()));
    }

    Ok(user)
}
