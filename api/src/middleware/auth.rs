//! Authentication middleware
//! Every request uses current PostgreSQL account authority and durable exact-token revocation.

use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{AppState, models::User, utils::verify_jwt};

/// Extractor for authenticated users
/// Reloads current database account state; Redis snapshots never grant identity.
impl FromRequestParts<AppState> for User {
    type Rejection = (StatusCode, &'static str);

    #[allow(clippy::manual_async_fn)]
    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
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
            // FIX-2026-04-26 (Priority 3): pass "access" expected_type so a refresh token
            // presented as a bearer access token is rejected.
            // Original line: let claims = verify_jwt(bearer.token(), &state.config.jwt_secret).map_err(|e| {
            let claims =
                verify_jwt(bearer.token(), &state.config.jwt_secret, "access").map_err(|e| {
                    tracing::warn!("JWT verification failed: {:?}", e);
                    (StatusCode::UNAUTHORIZED, "Invalid or expired token")
                })?;

            // Current DB state and durable token records are authoritative on every request.
            // Redis user snapshots and epochs may not grant access or restore revoked tokens.
            let account =
                crate::services::account_authority::authenticate(&state.db.pool, &claims).await?;
            parts
                .extensions
                .insert(crate::services::account_authority::capture(&claims)?);
            Ok(account.user)
        }
    }
}

/// Optional user extractor (for routes that work with or without auth)
#[allow(dead_code)]
pub struct OptionalUser(pub Option<User>);

impl FromRequestParts<AppState> for OptionalUser {
    type Rejection = (StatusCode, &'static str);

    #[allow(clippy::manual_async_fn)]
    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            match User::from_request_parts(parts, state).await {
                Ok(user) => Ok(OptionalUser(Some(user))),
                Err(_) => Ok(OptionalUser(None)),
            }
        }
    }
}
