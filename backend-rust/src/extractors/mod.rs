//! Custom Axum Extractors
//!
//! ICT 11+ Principal Engineer Grade

use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use uuid::Uuid;

use crate::{errors::AppError, models::User};

/// Authenticated user extractor
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = parts
            .extensions
            .get::<User>()
            .ok_or(AppError::Unauthorized("Authentication required".into()))?;

        Ok(AuthUser {
            user_id: user.id,
            email: user.email.clone(),
            role: user.role.clone(),
        })
    }
}
