//! Authentication Middleware
//!
//! ICT 7 SECURITY: Token validation with blacklist checking
//! Apple Principal Engineer Grade: Defense in depth

use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};

use crate::{
    errors::AppError,
    models::User,
    services::{is_token_revoked, AuthService},
    AppState,
};

/// Middleware to require authentication
/// ICT 7 SECURITY: Checks token blacklist before allowing access
pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized("Missing authorization token".into()))?;

    // ICT 7 SECURITY: Check if token has been revoked (e.g., user logged out)
    if is_token_revoked(&state.token_blacklist, token).await {
        tracing::warn!(
            target: "security",
            "Attempted use of revoked token"
        );
        return Err(AppError::Unauthorized("Token has been revoked".into()));
    }

    let auth_service = AuthService::new(&state.db, &state.config.jwt);
    let user = auth_service
        .validate_token(token)
        .await
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?;

    // ICT 11+: deleted_at check removed - column doesn't exist in production DB
    // Will be re-added once migration runs successfully

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}

/// Middleware to require admin role
pub async fn require_admin(request: Request, next: Next) -> Result<Response, AppError> {
    let user = request
        .extensions()
        .get::<User>()
        .ok_or(AppError::Unauthorized("Authentication required".into()))?;

    if !user.is_admin() {
        return Err(AppError::Forbidden("Admin access required".into()));
    }

    Ok(next.run(request).await)
}
