//! Trading Rooms routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - January 2026
//!
//! ICT 7 CORB Fix: All responses must be JSON with success field
//! to prevent CORB blocking cross-origin requests

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
    pub active_only: Option<bool>,
}

/// Query params for traders
#[derive(Debug, Deserialize)]
pub struct TradersQuery {
    pub room_slug: Option<String>,
    pub active_only: Option<bool>,
}

/// List trading rooms (public)
/// ICT 7: Returns hardcoded rooms until database table is ready
async fn list_trading_rooms(
    State(_state): State<AppState>,
    Query(_query): Query<TradingRoomsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Return hardcoded trading rooms
    // ICT 7: Proper response format with success field
    Ok(Json(json!({
        "success": true,
        "data": [
            {
                "id": 1,
                "name": "Day Trading Room",
                "slug": "day-trading-room",
                "type": "trading_room",
                "is_active": true,
                "is_featured": true,
                "sort_order": 1,
                "created_at": "2025-01-01T00:00:00Z",
                "updated_at": "2025-01-01T00:00:00Z"
            },
            {
                "id": 2,
                "name": "Explosive Swings",
                "slug": "explosive-swings",
                "type": "trading_room",
                "is_active": true,
                "is_featured": false,
                "sort_order": 2,
                "created_at": "2025-01-01T00:00:00Z",
                "updated_at": "2025-01-01T00:00:00Z"
            },
            {
                "id": 3,
                "name": "High Octane Scanner",
                "slug": "high-octane-scanner",
                "type": "trading_room",
                "is_active": true,
                "is_featured": false,
                "sort_order": 3,
                "created_at": "2025-01-01T00:00:00Z",
                "updated_at": "2025-01-01T00:00:00Z"
            }
        ],
        "meta": {
            "current_page": 1,
            "per_page": 20,
            "total": 3,
            "total_pages": 1
        }
    })))
}

/// List traders (admin)
/// ICT 7: Returns empty array until database table is ready
async fn list_traders(
    State(_state): State<AppState>,
    Query(_query): Query<TradersQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Return empty traders list
    // ICT 7: Proper response format with success field to prevent CORB
    Ok(Json(json!({
        "success": true,
        "data": [],
        "meta": {
            "current_page": 1,
            "per_page": 20,
            "total": 0,
            "total_pages": 0
        }
    })))
}

/// Public router for trading rooms
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_trading_rooms))
        .route("/traders", get(list_traders))
}

/// Admin router for trading rooms (includes traders management)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_trading_rooms))
        .route("/traders", get(list_traders))
}
