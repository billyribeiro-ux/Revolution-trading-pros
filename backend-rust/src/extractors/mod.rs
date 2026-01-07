//! Custom Axum Extractors
//!
//! ICT 11+ Principal Engineer Grade

use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

use crate::{errors::AppError, models::User};

/// Authenticated user extractor
/// ICT 11+: Production DB uses INT8 for user IDs
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i64,
    pub email: String,
    pub role: String,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    fn from_request_parts(parts: &mut Parts, _state: &S) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let result = parts
            .extensions
            .get::<User>()
            .map(|user| AuthUser {
                user_id: user.id,
                email: user.email.clone(),
                role: user.role.clone(),
            })
            .ok_or(AppError::Unauthorized("Authentication required".into()));

        std::future::ready(result)
    }
}
