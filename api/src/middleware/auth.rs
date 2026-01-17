//! Authentication middleware

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::{models::User, utils::verify_jwt, AppState};

/// Extractor for authenticated users
#[axum::async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract bearer token
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| {
                tracing::warn!("Missing or invalid Authorization header: {:?}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing or invalid authorization header",
                )
            })?;

        // Verify JWT
        let claims = verify_jwt(bearer.token(), &state.config.jwt_secret).map_err(|e| {
            tracing::warn!("JWT verification failed: {:?}", e);
            (StatusCode::UNAUTHORIZED, "Invalid or expired token")
        })?;

        // Get user from database
        let user: User = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(claims.sub)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error fetching user: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            })?
            .ok_or_else(|| {
                tracing::warn!("User not found for id: {}", claims.sub);
                (StatusCode::UNAUTHORIZED, "User not found")
            })?;

        Ok(user)
    }
}

/// Optional user extractor (for routes that work with or without auth)
#[allow(dead_code)]
pub struct OptionalUser(pub Option<User>);

#[axum::async_trait]
impl FromRequestParts<AppState> for OptionalUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        match User::from_request_parts(parts, state).await {
            Ok(user) => Ok(OptionalUser(Some(user))),
            Err(_) => Ok(OptionalUser(None)),
        }
    }
}
