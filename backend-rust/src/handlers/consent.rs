//! Consent Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::State, Json};

use crate::{errors::AppError, AppState};

pub async fn public_settings(State(_state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "enabled": true,
            "categories": ["necessary", "analytics", "marketing"]
        }
    })))
}
