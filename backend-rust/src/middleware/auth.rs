//! Authentication Middleware
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};

use crate::{errors::AppError, models::User, services::AuthService, AppState};

/// Middleware to require authentication
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
