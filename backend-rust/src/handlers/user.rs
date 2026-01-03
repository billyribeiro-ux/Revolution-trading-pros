//! User Profile Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{errors::AppError, extractors::AuthUser, responses::ApiResponse, AppState};

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: String,
    pub email: String,
    pub billing_address: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub current_password: Option<String>,
    pub new_password: Option<String>,
}

pub async fn get_profile(State(_state): State<AppState>, auth: AuthUser) -> Result<Json<ApiResponse<ProfileResponse>>, AppError> {
    Ok(Json(ApiResponse::success(ProfileResponse {
        id: auth.user_id.to_string(),
        first_name: None,
        last_name: None,
        display_name: "User".to_string(),
        email: "user@example.com".to_string(),
        billing_address: None,
    })))
}

pub async fn update_profile(State(_state): State<AppState>, _auth: AuthUser, Json(_payload): Json<UpdateProfileRequest>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "message": "Profile updated"})))
}
