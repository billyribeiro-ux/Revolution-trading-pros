//! Popup routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - January 2026

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::AppState;

/// Query params for active popups
#[derive(Debug, Deserialize)]
pub struct ActivePopupsQuery {
    pub page: Option<String>,
}

/// Get active popups for a page (public)
async fn get_active_popups(
    State(_state): State<AppState>,
    Query(query): Query<ActivePopupsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _page = query.page.unwrap_or_else(|| "/".to_string());
    
    // Return empty popups - no popups configured yet
    // This prevents 404 errors in the frontend
    Ok(Json(json!({
        "popups": []
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/active", get(get_active_popups))
}
