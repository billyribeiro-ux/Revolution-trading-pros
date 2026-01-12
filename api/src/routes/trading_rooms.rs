//! Trading Rooms routes - Revolution Trading Pros
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

/// Query params for trading rooms
#[derive(Debug, Deserialize)]
pub struct TradingRoomsQuery {
    pub with_counts: Option<bool>,
}

/// List trading rooms (public)
async fn list_trading_rooms(
    State(_state): State<AppState>,
    Query(_query): Query<TradingRoomsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Return empty trading rooms - feature not yet implemented
    // This prevents 404 errors in the frontend
    Ok(Json(json!({
        "data": [],
        "meta": {
            "current_page": 1,
            "per_page": 20,
            "total": 0,
            "total_pages": 0
        }
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_trading_rooms))
}
