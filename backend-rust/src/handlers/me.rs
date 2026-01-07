//! Current User Handlers (/me)
//!
//! ICT 11+ Principal Engineer Grade
//! User self-service endpoints

use axum::{extract::State, Json};
use serde::Serialize;

use crate::{
    errors::AppError, extractors::AuthUser, responses::ApiResponse, services::UserService, AppState,
};

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: String,
    pub name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub is_verified: bool,
    pub is_admin: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct MembershipResponse {
    pub id: String,
    pub plan_name: String,
    pub status: String,
    pub started_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub product_type: String,
}

/// GET /api/me - Get current user profile
#[tracing::instrument(skip(state))]
pub async fn show(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ApiResponse<MeResponse>>, AppError> {
    let user_service = UserService::new(&state.db);
    let user = user_service
        .find_by_id(auth_user.user_id)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    Ok(Json(ApiResponse::success(MeResponse {
        id: user.id.to_string(),
        name: user.name.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        email: user.email.clone(),
        avatar_url: user.avatar_url.clone(),
        role: user.role.to_string(),
        is_verified: user.email_verified_at.is_some(),
        is_admin: user.is_admin(),
        created_at: user.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    })))
}

/// PUT /api/me - Update current user profile
#[tracing::instrument(skip(state, payload))]
pub async fn update(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<MeResponse>>, AppError> {
    let user_service = UserService::new(&state.db);

    // Extract fields from payload
    let name = payload.get("name").and_then(|v| v.as_str());
    let first_name = payload.get("firstName").and_then(|v| v.as_str());
    let last_name = payload.get("lastName").and_then(|v| v.as_str());
    let avatar_url = payload.get("avatar_url").and_then(|v| v.as_str());

    let user = user_service
        .update_profile(auth_user.user_id, name, first_name, last_name, avatar_url)
        .await?;

    Ok(Json(ApiResponse::success(MeResponse {
        id: user.id.to_string(),
        name: user.name.clone(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        email: user.email.clone(),
        avatar_url: user.avatar_url.clone(),
        role: user.role.to_string(),
        is_verified: user.email_verified_at.is_some(),
        is_admin: user.is_admin(),
        created_at: user.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    })))
}

/// GET /api/me/memberships - Get user's active memberships
#[tracing::instrument(skip(state))]
pub async fn memberships(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ApiResponse<Vec<MembershipResponse>>>, AppError> {
    let user_service = UserService::new(&state.db);
    let memberships = user_service.get_memberships(auth_user.user_id).await?;

    let response: Vec<MembershipResponse> = memberships
        .into_iter()
        .map(|m| MembershipResponse {
            id: m.id.to_string(),
            plan_name: m.plan_name,
            status: m.status,
            started_at: m.started_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            expires_at: m
                .expires_at
                .map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()),
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

/// GET /api/me/products - Get user's purchased products
#[tracing::instrument(skip(state))]
pub async fn products(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, AppError> {
    let user_service = UserService::new(&state.db);
    let products = user_service.get_products(auth_user.user_id).await?;

    let response: Vec<ProductResponse> = products
        .into_iter()
        .map(|p| ProductResponse {
            id: p.id.to_string(),
            name: p.name,
            slug: p.slug,
            product_type: p.product_type,
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

/// GET /api/me/indicators - Get user's indicators
#[tracing::instrument(skip(state))]
pub async fn indicators(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, AppError> {
    let user_service = UserService::new(&state.db);
    let indicators = user_service.get_indicators(auth_user.user_id).await?;

    let response: Vec<ProductResponse> = indicators
        .into_iter()
        .map(|p| ProductResponse {
            id: p.id.to_string(),
            name: p.name,
            slug: p.slug,
            product_type: "indicator".to_string(),
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}
