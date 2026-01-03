//! Analytics Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{errors::AppError, AppState};

#[derive(Debug, Deserialize)]
pub struct TrackEventRequest {
    pub event: String,
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct PageViewRequest {
    pub url: String,
    pub referrer: Option<String>,
}

pub async fn track_event(State(_state): State<AppState>, Json(_payload): Json<TrackEventRequest>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true})))
}

pub async fn track_pageview(State(_state): State<AppState>, Json(_payload): Json<PageViewRequest>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true})))
}
