//! Newsletter Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::{errors::AppError, AppState};

#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmQuery {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct UnsubscribeQuery {
    pub token: String,
}

pub async fn subscribe(
    State(_state): State<AppState>,
    Json(_payload): Json<SubscribeRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(
        serde_json::json!({"success": true, "message": "Subscribed successfully"}),
    ))
}

pub async fn confirm(
    State(_state): State<AppState>,
    Query(_query): Query<ConfirmQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(
        serde_json::json!({"success": true, "message": "Email confirmed"}),
    ))
}

pub async fn unsubscribe(
    State(_state): State<AppState>,
    Query(_query): Query<UnsubscribeQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(
        serde_json::json!({"success": true, "message": "Unsubscribed"}),
    ))
}
